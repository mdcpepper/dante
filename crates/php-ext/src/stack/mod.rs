//! Promotion Stack/Graph and Layers/Nodes

use ext_php_rs::{
    class::RegisteredClass,
    convert::{FromZval, IntoZval},
    exception::PhpException,
    flags::DataType,
    prelude::*,
    types::Zval,
    zend::ce,
};
use slotmap::SlotMap;

use lattice::{
    graph::{GraphError, PromotionGraph, PromotionGraphBuilder},
    promotions::{PromotionKey, promotion},
};

use crate::{
    promotions::direct_discount::DirectDiscountPromotion,
    stack::layers::{Layer, LayerOutput, LayerRef},
};

pub mod layers;

/// Exception thrown when a stack or layer configuration is invalid
#[derive(Default)]
#[php_class]
#[php(
    name = "FeedCode\\Lattice\\Stack\\InvalidStackException",
    extends(ce = ce::exception, stub = "\\Exception")
)]
pub struct InvalidStackException;

#[php_impl]
impl InvalidStackException {}

#[derive(Debug, Clone)]
#[php_class]
#[php(name = "FeedCode\\Lattice\\Stack")]
pub struct Stack {
    #[php(prop)]
    layers: Vec<LayerRef>,
}

#[php_impl]
impl Stack {
    pub fn __construct(layers: Option<Vec<LayerRef>>) -> Self {
        Self {
            layers: layers.unwrap_or_default(),
        }
    }

    pub fn validate_graph(&self) -> PhpResult<bool> {
        self.try_to_core_graph()?;

        Ok(true)
    }
}

impl Stack {
    pub(crate) fn try_to_core_graph(&self) -> Result<PromotionGraph<'static>, PhpException> {
        if self.layers.is_empty() {
            return Err(PhpException::from_class::<InvalidStackException>(
                "Stack must contain at least one layer.".to_string(),
            ));
        }

        let mut builder = PromotionGraphBuilder::new();
        let mut promotion_keys = SlotMap::<PromotionKey, ()>::with_key();
        let mut layer_nodes = Vec::with_capacity(self.layers.len());

        for (idx, layer_ref) in self.layers.iter().enumerate() {
            let layer: Layer = layer_ref.try_into()?;
            let output_mode = layer.output();

            if output_mode == LayerOutput::Split {
                return Err(PhpException::from_class::<InvalidStackException>(
                    "LayerOutput::Split is not supported in linear Stack yet.".to_string(),
                ));
            }

            let mut core_promotions = Vec::with_capacity(layer.promotions().len());

            for promo in layer.promotions() {
                let promotion_key = promotion_keys.insert(());
                let promo: DirectDiscountPromotion = promo.try_into()?;

                core_promotions.push(promotion(promo.try_to_core_with_key(promotion_key)?));
            }

            let node = builder
                .add_layer(format!("Layer {idx}"), core_promotions, output_mode.into())
                .map_err(graph_error_to_php_exception)?;

            layer_nodes.push(node);
        }

        if let Some(root) = layer_nodes.first().copied() {
            builder.set_root(root);
        }

        for edge in layer_nodes.windows(2) {
            builder
                .connect_pass_through(edge[0], edge[1])
                .map_err(graph_error_to_php_exception)?;
        }

        PromotionGraph::from_builder(builder).map_err(graph_error_to_php_exception)
    }
}

fn graph_error_to_php_exception(error: GraphError) -> PhpException {
    PhpException::from_class::<InvalidStackException>(format!("Unable to build stack: {error}"))
}

#[derive(Debug)]
pub struct StackRef(Zval);

impl StackRef {
    pub fn from_stack(stack: Stack) -> Self {
        let mut zv = Zval::new();

        stack
            .set_zval(&mut zv, false)
            .expect("stack should always convert to object zval");

        Self(zv)
    }
}

impl<'a> FromZval<'a> for StackRef {
    const TYPE: DataType = DataType::Object(Some(<Stack as RegisteredClass>::CLASS_NAME));

    fn from_zval(zval: &'a Zval) -> Option<Self> {
        let obj = zval.object()?;

        if obj.is_instance::<Stack>() {
            Some(Self(zval.shallow_clone()))
        } else {
            None
        }
    }
}

impl Clone for StackRef {
    fn clone(&self) -> Self {
        Self(self.0.shallow_clone())
    }
}

impl IntoZval for StackRef {
    const TYPE: DataType = DataType::Object(Some(<Stack as RegisteredClass>::CLASS_NAME));
    const NULLABLE: bool = false;

    fn set_zval(self, zv: &mut Zval, persistent: bool) -> ext_php_rs::error::Result<()> {
        self.0.set_zval(zv, persistent)
    }
}

impl TryFrom<&StackRef> for Stack {
    type Error = PhpException;

    fn try_from(value: &StackRef) -> Result<Self, Self::Error> {
        let Some(obj) = value.0.object() else {
            return Err(PhpException::from_class::<InvalidStackException>(
                "Stack object is invalid.".to_string(),
            ));
        };

        let layers = obj.get_property::<Vec<LayerRef>>("layers").map_err(|_| {
            PhpException::from_class::<InvalidStackException>(
                "Stack layers property is invalid.".to_string(),
            )
        })?;

        Ok(Self { layers })
    }
}

impl TryFrom<StackRef> for Stack {
    type Error = PhpException;

    fn try_from(value: StackRef) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
