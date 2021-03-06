# Nodes collection

The `Nodes` collection consists of `Subgraphs` and `Agents`. A `Subgraph` or an `Agent` may be referred to as a `Node`.

## Agents

### What?

Executable `Subgraphs` are defined as a network of `Agents`, which exchange typed data across predefined connections by message passing, where the connections are specified externally to the processes. These `Agents`  can be reconnected endlessly to form different executable `Subgraphs` without having to be changed internally.

### Why?

Functions in a programming language should be placed in a content addressable store, this is the horizontal plane. The vertical plane should be constructed using unique addresses into this content addressable store, critically each address should solve a single problem, and may do so by referencing multiple other unique addresses in the content addressable store. Users must not have knowledge of these unique addresses but a translation process should occur from a human readable name to a universally unique address. Read [more](http://erlang.org/pipermail/erlang-questions/2011-May/058768.html) about the problem.

Nix gives us the content addressable store which allows for `reproducibility`, and these `agents` give us `reusablility`. The combination is particularly potent form of programming.

Once you have the above, you have truly reusable and reproducible functions. Fractalide nodes are just this, and it makes the below so much easier to achieve:

```
* Open source collaboration
* Open peer review of nodes
* Nice clean reusable nodes
* Reproducible applications
```

### Who?

Typically programmers will develop `Agents`. They specialize in making `Agents` as efficient and reusable as possible, while people who focus on the Science give the requirements and use the `Subgraphs`. Just as a hammer is designed to be reused, so `Subgraphs` and `Agents` should be designed for reuse.

### Where?

The `Agents` are found in this `nodes` directory, or the `nodes` directory of a [fractal](../fractals/README.md).

```
processchunk
├── default.nix
├── agg_chunk_triples
│   ├── default.nix <---
│   └── lib.rs
├── convert_json_vector
│   ├── default.nix <---
│   └── lib.rs
├── extract_keyvalue
│   ├── default.nix <---
│   └── lib.rs
├── file_open
│   ├── default.nix <---
│   └── lib.rs
└── iterate_paths
    ├── default.nix <---
    └── lib.rs
```
Typically when you see a `lib.rs` in the same directory as a `default.nix` you know it's an `Agent`.

### How?

An `Agent` consists of two parts:
* a `nix` `default.nix` file that sets up an environment to satisfy `rustc`.
* a `rust` `lib.rs` file implements your `agent`.

#### The `agent` Nix function.

The `agent` function in the `default.nix` requires you make decisions about three types of dependencies.
* What `edges` are needed?
* What `crates` from [crates.io](https://crates.io) are needed?
* What `osdeps` or `operating system level dependencies` are needed?

``` nix
{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_bool ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [ openssl ];
}
```

* The `{ agent, edges, crates, pkgs }:` lambda imports: The `edges` attribute which consists of every `edge` available on the system. The `crates` attribute set consists of every `crate` on https://crates.io. Lastly the `pkgs` pulls in every third party package available on NixOS, here's the whole [list](http://nixos.org/nixos/packages.html).
* The `agent` function builds the rust `lib.rs` source code, and accepts these arguments:
  * The `src` attribute is used to derive an `Agent` name based on location in the directory hierarchy.
  * The `edges` lazily compiles schema and composite schema ensuring their availability.
  * The `crates` specifies exactly which `crates` are needed in scope.
  * The `osdeps` specifies exactly which `pkgs`, or third party `operating system level libraries` such as `openssl` needed in scope.

Only specified dependencies and their transitive dependencies will be pulled into scope once the `agent` compilation starts.

This is the output of the above `agent`'s compilation:

```
/nix/store/dp8s7d3p80q18a3pf2b4dk0bi4f856f8-maths_boolean_nand
└── lib
    └── libagent.so
```

#### The `agent!` Rust macro

This is the heart of `Fractalide`. Everything revolves around this `API`. The below is an implementation of the `${maths_boolean_nand}` `agent` seen earlier.

``` rust
#[macro_use]
extern crate rustfbp;
extern crate capnp;

agent! {
  input(a: prim_bool, b: prim_bool),
  output(output: prim_bool),
  fn run(&mut self) -> Result<Signal> {
    let a = {
      let mut ip_a = self.input.a.recv()?;
      let a_reader: prim_bool::Reader = ip_a.read_schema()?;
      a_reader.get_boolean()
    };
    let b = {
      let mut ip_b = self.input.b.recv()?;
      let b_reader: prim_bool::Reader = ip_b.read_schema()?;
      b_reader.get_boolean()
    };

    let mut out_ip = IP::new();
    {
      let mut boolean = out_ip.build_schema::<prim_bool::Builder>();
      boolean.set_boolean(if a == true && b == true {false} else {true});
    }
    self.output.output.send(out_ip)?;
    Ok(End)
  }
}
```

An explanation of each of the items should be given.
All expresions are optional except for the `run` function.

##### `input`:
``` rust
agent! {
  input(input_name: prim_bool),
  fn run(&mut self) -> Result<Signal> {
    let a = {
      let mut a_msg = self.input.input_name.recv()?;
      let a_reader: prim_bool::Reader = a_msg.read_schema()?;
      a_reader.get_boolean()
    };
    Ok(End)
  }
}
```
The `input` port, is a bounded buffer simple input channel that carries Cap'n Proto schemas as messages.

##### `inarr`:
``` rust
agent! {
  inarr(input_array_name: prim_bool),
  fn run(&mut self) -> Result<Signal> {
    for ins in self.inarr.input_array_name.values() {
      let a = {
        let mut a_msg = ins.recv()?;
        let a_reader: prim_bool::Reader = a_msg.read_schema()?;
        a_reader.get_boolean()
      };
    }
    Ok(End)
  }
}
```
The `inarr` is an input array port, which consists of multiple elements of a port.
They are used when the `Subgraph` developer needs multiple elements of a port, for example an `adder` has multiple input elements. This `adder` `agent` may be used in many scenarios where the amount of inputs are unknown at `agent development time`.

##### `output`:
``` rust
agent! {
  output(output_name: prim_bool),
  fn run(&mut self) -> Result<Signal> {
    let mut msg_out = msg::new();
    {
      let mut boolean = msg_out.build_schema::<prim_bool::Builder>();
      boolean.set_boolean(true);
    }
    self.output.output_name.send(msg_out)?;
    Ok(End)
  }
}
```
The humble simple output port. It doesn't have elements and is fixed at `subgraph development time`.
##### `outarr`:
``` rust
agent! {
  input(input: any),
  outarr(clone: any),
  fn run(&mut self) -> Result<Signal> {
    let msg = self.input.input.recv()?;
    for p in self.outarr.clone.elements()? {
        self.outarr.clone.send( &p, msg.clone())?;
    }
    Ok(End)
  }
}
```
The `outarr` port is an `output array port`. It contains elements which may be expanded at `subgraph development time`.

##### `state`:
``` rust
#[macro_use]
extern crate rustfbp;
extern crate capnp;
extern crate nanomsg;

use nanomsg::{Socket, Protocol};
pub struct State {
    socket: Option<Socket>,
}

impl State {
    fn new() -> State {
        State {
            socket: None,
        }
    }
}

agent! {
  input(connect: prim_text, ip: any),
  state(State => State::new()),
  fn run(&mut self) -> Result<Signal> {
    if let Ok(mut ip) = self.inputs.connect.try_recv() {
        let reader: prim_text::Reader = ip.read_schema()?;
        let mut socket = Socket::new(Protocol::Push)
            .or(Err(result::Error::Misc("Cannot create socket".into())))?;
        socket.bind(reader.get_text()?)
            .or(Err(result::Error::Misc("Cannot connect socket".into())))?;
        self.state.socket = Some(socket);
    }

    if let Ok(ip) = self.inputs.ip.try_recv() {
        if let Some(ref mut socket) = self.state.socket {
            socket.write(&ip.vec[..]);
        }
    }
    Ok(End)
  }
}
```

It is basically the state of the agent. A `State` allows us to keep complex state hanging around if needed. It can be any Rust type. The `state` is persistant for all the executions, so each time you are in the function `run()`, you can access and modify it. 

##### `option`:
``` rust
agent! {
  option(prim_bool),
  fn run(&mut self) -> Result<Signal> {
    let mut opt = self.option.recv();
    let opt_reader: prim_bool::Reader = opt.read_schema()?;
    let opt_boolean = opt_reader.get_boolean()?;
    Ok(End)
  }
}
```
The `option` port gives the `subgraph` developer a way to send in parameters such as a connection string and the message will not be consumed and thrown away, that message may be read on every function run. Whereas other ports will consume and throw away the message.

##### `accumulator`:
``` rust
agent! {
  accumulator(prim_bool),
  fn run(&mut self) -> Result<Signal> {
    let mut acc = self.ports.accumulator.recv()?;
    let acc_reader: prim_bool::Reader = ip_acc.read_schema()?;
    let acc_boolean = acc_reader.get_boolean()?;
    Ok(End)
  }
}
```
The `accumulator` gives the `subgraph` developer a way to start counting at a certain number. This port isn't used so often.
##### `run`:
This function does the actual processing and is the only mandatory expression of this macro.

Now that you've had a basic introduction to the `Nodes` collection, you might want to head on over to

1. [Edges](../../edges/README.md)
2. [Fractals](../../fractals/README.md)
3. [Services](../../services/README.md)
4. [HOWTO](../../HOWTO.md)
