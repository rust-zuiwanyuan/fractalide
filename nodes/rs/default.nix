{ buffet }:
let

callPackage = buffet.pkgs.lib.callPackageWith ( buffet.pkgs // buffet.support.rs // buffet.support // buffet.mods // buffet);
# Insert in alphabetical order in relevant section to reduce conflicts
# This system grows by the slow accretion of nodes.
# Once a node is declared stable, it is _/forbidden/_ to:
# * change the node name
# * change any port name
# * delete a node, or port
# * change or delete items in the schema on any port, though additions of schema items are acceptable
# * backtrack towards the experimental category
# Once a node becomes stable, all associated schema also become stable.
# Nodes should function reliably decades from now.
# Changing names after experimental upgrade is a breaking change, we do not do that here.
self = rec {

  # RAW NODES
  # -   are incomplete and immature, they may wink into and out of existance
  # -   use at own risk, anything in this section can change at any time.

  app_todo_nodes = buffet.fractals.app_todo.nodes;
  app_todo_model_test = buffet.fractals.app_todo_model.nodes.test;
  app_todo_controller_test = buffet.fractals.app_todo_controller.nodes.test;
  bench = callPackage ./bench {};
  bench_load = callPackage ./bench/load {};
  bench_inc_1000 = callPackage ./bench/inc_1000 {};
  bench_inc = callPackage ./bench/inc {};
  debug = callPackage ./debug {};
  docs = callPackage ./docs {};
  example_wrangle = buffet.fractals.example_wrangle.nodes.example_wrangle;
  nanomsg_nodes = buffet.fractals.nanomsg.nodes;
  nanomsg_test = buffet.fractals.nanomsg.nodes.test;
  maths_boolean_and = callPackage ./maths/boolean/and {};
  maths_boolean_nand = callPackage ./maths/boolean/nand {};
  maths_boolean_not = callPackage ./maths/boolean/not {};
  maths_boolean_or = callPackage ./maths/boolean/or {};
  maths_boolean_print = callPackage ./maths/boolean/print {};
  maths_boolean_xor = callPackage ./maths/boolean/xor {};
  maths_number_add = callPackage ./maths/number/add {};
  net_http_nodes = buffet.fractals.net_http.nodes;
  net_http_test = buffet.fractals.net_http.nodes.test;
  net_ndn = buffet.fractals.net_ndn.nodes.ndn;
  net_ndn_test = buffet.fractals.net_ndn.nodes.test;
  test_nand = callPackage ./test/nand {};
  test_edges = callPackage ./test/edges {};
  ui_js_nodes = buffet.fractals.ui_js.nodes;
  app_growtest = buffet.fractals.ui_js.nodes.app_growtest;
  web_server = callPackage ./web/server {};
  workbench = buffet.fractals.workbench.nodes.workbench;
  workbench_test = buffet.fractals.workbench.nodes.test;

  # DRAFT NODES
  # -   draft nodes change a lot in tandom with other nodes in their subgraph
  # -   there will be change in these nodes
  # -   few people are using these nodes so expect breakage

  fs_list_dir = callPackage ./fs/list/dir {};
  fs_file_open = callPackage ./fs/file/open {};
  halter = callPackage ./halter {};
  io_print = callPackage ./io/print {};
  msg_action = callPackage ./msg/action {};
  msg_clone = callPackage ./msg/clone {};
  msg_delay = callPackage ./msg/delay {};
  msg_dispatcher = callPackage ./msg/dispatcher {};
  msg_replace = callPackage ./msg/replace {};

  # STABLE NODES
  # -   do not change names of ports, agents nor subgraphs,
  # -   you may add new port names, but never change, nor remove port names
  # -   never change or remove schema names
  # -   you may add new schema items S

  # DEPRECATED NODES
  # -   do not change names of ports, agents nor subgraphs.
  # -   keep the implemenation functioning
  # -   print a warning message and tell users to use replacement node

  # LEGACY NODES
  # -   do not change names of ports, agents nor subgraphs.
  # -   assert and remove implementation
};
in
self
