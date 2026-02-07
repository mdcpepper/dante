= ILP Formulations for Layered Promotion Graph

== Layer 0: supplier-promotions (Node NodeIndex(0))

_Note: This layer contains 1 promotion variable(s)._

== Decision Variables

All decision variables are binary.

=== Presence Variables (Full Price)

- $x_1$: Item 1 (Coca Cola 500ml) at full price (250)
- $x_2$: Item 2 (Chicken Sandwich) at full price (400)
- $x_3$: Item 3 (Water) at full price (100)
- $x_4$: Item 4 (Crisps) at full price (150)
- $x_5$: Item 5 (Newspaper) at full price (200)

=== Promotion Variables (Participation & Discounts)

- $y_1$: Item 1 with promotion "Supplier: 10% Off Coke" (225)

== Objective Function

Minimize:

$ "minimize" quad 250 dot x_1 + 400 dot x_2 + 100 dot x_3 $
$ quad + 150 dot x_4 + 200 dot x_5 + 225 dot y_1 $

== Constraints

=== Exclusivity Constraints

Each item must be purchased exactly once (at full price OR discounted by a single promotion):

$ x_1 + y_1 = 1 $ (Item 1 (Coca Cola 500ml))

$ x_2 = 1 $ (Item 2 (Chicken Sandwich))

$ x_3 = 1 $ (Item 3 (Water))

$ x_4 = 1 $ (Item 4 (Crisps))

$ x_5 = 1 $ (Item 5 (Newspaper))


== Full ILP in Standard Form

$ "minimize" quad 250 dot x_1 + 400 dot x_2 + 100 dot x_3 $
$ quad + 150 dot x_4 + 200 dot x_5 + 225 dot y_1 $

$ "subject to" quad x_1 + y_1 = 1 $
$ quad x_2 = 1 $
$ quad x_3 = 1 $
$ quad x_4 = 1 $
$ quad x_5 = 1 $

$ x_i in {0,1} $

#pagebreak()

== Layer 1: base-promotions (Node NodeIndex(2))

_Note: This layer contains 8 promotion variable(s)._

== Decision Variables

All decision variables are binary.

=== Presence Variables (Full Price)

- $x_1$: Item 1 (Coca Cola 500ml) at full price (225)
- $x_2$: Item 2 (Chicken Sandwich) at full price (400)
- $x_3$: Item 3 (Water) at full price (100)
- $x_4$: Item 4 (Crisps) at full price (150)
- $x_5$: Item 5 (Newspaper) at full price (200)

=== Promotion Variables (Participation & Discounts)

- $y_1$: Item 1 with promotion "Buy One Get One Free Drinks" (225) [participation]
- $y_2$: Item 1 with promotion "Buy One Get One Free Drinks" (0) [discount]
- $y_3$: Item 3 with promotion "Buy One Get One Free Drinks" (100) [participation]
- $y_4$: Item 3 with promotion "Buy One Get One Free Drinks" (0) [discount]
- $y_5$: Item 1 with promotion "£5 Meal Deal" (0) [slot]
- $y_6$: Item 2 with promotion "£5 Meal Deal" (0) [slot]
- $y_7$: Item 3 with promotion "£5 Meal Deal" (0) [slot]
- $y_8$: Item 4 with promotion "£5 Meal Deal" (0) [slot]

=== Auxiliary Variables

- $a_1$: bundle count for promotion "£5 Meal Deal"
- $s_1$: DFA state for promotion "Buy One Get One Free Drinks" (pos=0, state=0)
- $t_1$: DFA take for promotion "Buy One Get One Free Drinks" (pos=0, state=0)
- $s_2$: DFA state for promotion "Buy One Get One Free Drinks" (pos=0, state=1)
- $t_2$: DFA take for promotion "Buy One Get One Free Drinks" (pos=0, state=1)
- $s_3$: DFA state for promotion "Buy One Get One Free Drinks" (pos=1, state=0)
- $t_3$: DFA take for promotion "Buy One Get One Free Drinks" (pos=1, state=0)
- $s_4$: DFA state for promotion "Buy One Get One Free Drinks" (pos=1, state=1)
- $t_4$: DFA take for promotion "Buy One Get One Free Drinks" (pos=1, state=1)
- $s_5$: DFA state for promotion "Buy One Get One Free Drinks" (pos=2, state=0)
- $s_6$: DFA state for promotion "Buy One Get One Free Drinks" (pos=2, state=1)

== Objective Function

Minimize:

$ "minimize" quad 225 dot x_1 + 400 dot x_2 + 100 dot x_3 $
$ quad + 150 dot x_4 + 200 dot x_5 + 225 dot y_1 $
$ quad - 225 dot y_2 + 100 dot y_3 - 100 dot y_4 $
$ quad + 500 dot a_1 $

== Constraints

=== Exclusivity Constraints

Each item must be purchased exactly once (at full price OR discounted by a single promotion):

$ x_1 + y_1 + y_5 = 1 $ (Item 1 (Coca Cola 500ml))

$ x_2 + y_6 = 1 $ (Item 2 (Chicken Sandwich))

$ x_3 + y_3 + y_7 = 1 $ (Item 3 (Water))

$ x_4 + y_8 = 1 $ (Item 4 (Crisps))

$ x_5 = 1 $ (Item 5 (Newspaper))


=== Promotion Constraints

$ y_6 - a_1 >= 0 $ (slot min for promotion "£5 Meal Deal")

$ y_6 - a_1 <= 0 $ (slot max for promotion "£5 Meal Deal")

$ y_5 + y_7 - a_1 >= 0 $ (slot min for promotion "£5 Meal Deal")

$ y_5 + y_7 - a_1 <= 0 $ (slot max for promotion "£5 Meal Deal")

$ y_8 - a_1 >= 0 $ (slot min for promotion "£5 Meal Deal")

$ y_8 - a_1 <= 0 $ (slot max for promotion "£5 Meal Deal")

$ s_1 + s_2 = 1 $ (DFA state uniqueness for promotion "Buy One Get One Free Drinks")

$ s_3 + s_4 = 1 $ (DFA state uniqueness for promotion "Buy One Get One Free Drinks")

$ s_5 + s_6 = 1 $ (DFA state uniqueness for promotion "Buy One Get One Free Drinks")

$ s_1 = 1 $ (DFA initial state for promotion "Buy One Get One Free Drinks")

$ s_5 = 1 $ (DFA final state for promotion "Buy One Get One Free Drinks")

$ - s_1 + s_3 + t_1 - t_2 = 0 $ (DFA state transition for promotion "Buy One Get One Free Drinks")

$ - s_2 + s_4 - t_1 + t_2 = 0 $ (DFA state transition for promotion "Buy One Get One Free Drinks")

$ - s_3 + s_5 + t_3 - t_4 = 0 $ (DFA state transition for promotion "Buy One Get One Free Drinks")

$ - s_4 + s_6 - t_3 + t_4 = 0 $ (DFA state transition for promotion "Buy One Get One Free Drinks")

$ y_1 - t_1 - t_2 = 0 $ (DFA link participation for promotion "Buy One Get One Free Drinks")

$ y_3 - t_3 - t_4 = 0 $ (DFA link participation for promotion "Buy One Get One Free Drinks")

$ y_2 - t_2 = 0 $ (DFA link discount for promotion "Buy One Get One Free Drinks")

$ y_4 - t_4 = 0 $ (DFA link discount for promotion "Buy One Get One Free Drinks")

$ - s_1 + t_1 <= 0 $ (DFA restrict transitions for promotion "Buy One Get One Free Drinks")

$ - s_2 + t_2 <= 0 $ (DFA restrict transitions for promotion "Buy One Get One Free Drinks")

$ - s_3 + t_3 <= 0 $ (DFA restrict transitions for promotion "Buy One Get One Free Drinks")

$ - s_4 + t_4 <= 0 $ (DFA restrict transitions for promotion "Buy One Get One Free Drinks")


== Full ILP in Standard Form

$ "minimize" quad 225 dot x_1 + 400 dot x_2 + 100 dot x_3 $
$ quad + 150 dot x_4 + 200 dot x_5 + 225 dot y_1 $
$ quad - 225 dot y_2 + 100 dot y_3 - 100 dot y_4 $
$ quad + 500 dot a_1 $

$ "subject to" quad x_1 + y_1 + y_5 = 1 $
$ quad x_2 + y_6 = 1 $
$ quad x_3 + y_3 + y_7 = 1 $
$ quad x_4 + y_8 = 1 $
$ quad x_5 = 1 $
$ quad y_6 - a_1 >= 0 $
$ quad y_6 - a_1 <= 0 $
$ quad y_5 + y_7 - a_1 >= 0 $
$ quad y_5 + y_7 - a_1 <= 0 $
$ quad y_8 - a_1 >= 0 $
$ quad y_8 - a_1 <= 0 $
$ quad s_1 + s_2 = 1 $
$ quad s_3 + s_4 = 1 $
$ quad s_5 + s_6 = 1 $
$ quad s_1 = 1 $
$ quad s_5 = 1 $
$ quad - s_1 + s_3 + t_1 - t_2 = 0 $
$ quad - s_2 + s_4 - t_1 + t_2 = 0 $
$ quad - s_3 + s_5 + t_3 - t_4 = 0 $
$ quad - s_4 + s_6 - t_3 + t_4 = 0 $
$ quad y_1 - t_1 - t_2 = 0 $
$ quad y_3 - t_3 - t_4 = 0 $
$ quad y_2 - t_2 = 0 $
$ quad y_4 - t_4 = 0 $
$ quad - s_1 + t_1 <= 0 $
$ quad - s_2 + t_2 <= 0 $
$ quad - s_3 + t_3 <= 0 $
$ quad - s_4 + t_4 <= 0 $

$ x_i in {0,1} $

#pagebreak()

== Layer 2: staff-discount (Node NodeIndex(1))

_Note: This layer contains 2 promotion variable(s)._

== Decision Variables

All decision variables are binary.

=== Presence Variables (Full Price)

- $x_1$: Item 1 (Coca Cola 500ml) at full price (100)
- $x_2$: Item 2 (Chicken Sandwich) at full price (200)

=== Promotion Variables (Participation & Discounts)

- $y_1$: Item 1 with promotion "5% Staff discount" (95)
- $y_2$: Item 2 with promotion "5% Staff discount" (190)

== Objective Function

Minimize:

$ "minimize" quad 100 dot x_1 + 200 dot x_2 + 95 dot y_1 $
$ quad + 190 dot y_2 $

== Constraints

=== Exclusivity Constraints

Each item must be purchased exactly once (at full price OR discounted by a single promotion):

$ x_1 + y_1 = 1 $ (Item 1 (Coca Cola 500ml))

$ x_2 + y_2 = 1 $ (Item 2 (Chicken Sandwich))


== Full ILP in Standard Form

$ "minimize" quad 100 dot x_1 + 200 dot x_2 + 95 dot y_1 $
$ quad + 190 dot y_2 $

$ "subject to" quad x_1 + y_1 = 1 $
$ quad x_2 + y_2 = 1 $

$ x_i in {0,1} $
