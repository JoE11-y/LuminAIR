use luminair_graph::{graph::LuminairGraph, StwoCompiler};
use luminal::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cx = Graph::new();

    // ======= Define initializers =======
    let a = cx.tensor((2, 2)).set(vec![1.0, 2.0, 3.0, 4.0]);
    let b = cx.tensor((2, 2)).set(vec![10.0, 20.0, 30.0, 40.0]);
    let w = cx.tensor((2, 2)).set(vec![-1.0, -1.0, -1.0, -1.0]);

    // ======= Define graph =======
    let c = a * b;
    let d = c + w;
    let mut e = (c * d).retrieve();

    // ======= Compile graph =======
    println!("Compiling computation graph...");
    cx.compile(<(GenericCompiler, StwoCompiler)>::default(), &mut e);
    println!("Graph compiled successfully. ✅");

    // cx.display();

    // ======= Execute graph & generate trace =======
    println!("Executing graph and generating execution trace...");
    let trace = cx.gen_trace();
    println!("Execution trace generated successfully. ✅");
    let result = cx.get_output(e.id);
    println!("Final result: {:?}", result);

    // ======= Prove & Verify =======
    println!("Generating proof for execution trace...");
    let proof = cx.prove(trace)?;
    println!("Proof generated successfully. ✅");

    println!("Verifying proof...");
    cx.verify(proof)?;
    println!("Proof verified successfully. Computation integrity ensured. 🎉");

    Ok(())
}
