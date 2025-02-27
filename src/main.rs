#![allow(dead_code)]

mod geometry;
mod mesh;
mod utils;

use geometry::prelude::*;
use mesh::nodes::Nodes;
use utils::plotting::plot_nodes_2d;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create uniform mesh
    // let mut vertices_uniform = Vertices::new_2d(200, 100);
    // create_busemann_mesh_2d(
    //     &mut vertices_uniform,
    //     2.0,  // length in x direction
    //     WallDistribution::Uniform,
    //     inlet_contour,
    //     None,
    // );
    // let mut nodes_uniform = Nodes::new_2d();
    // nodes_uniform.populate(&vertices_uniform).expect("failed to populate uniform nodes");
    // plot_nodes_2d(&nodes_uniform, "busemann_nodes_uniform.png", false)?;

    // create hyperbolic tangent mesh
    // let mut vertices_tanh = Vertices::new_2d(200, 100);
    // create_busemann_mesh_2d(
    //     &mut vertices_tanh,
    //     2.0,
    //     WallDistribution::HyperbolicTangent,
    //     inlet_contour,
    //     Some(1.5),  // beta parameter for clustering
    // );
    // let mut nodes_tanh = Nodes::new_2d();
    // nodes_tanh.populate(&vertices_tanh).expect("failed to populate tanh nodes");
    // plot_nodes_2d(&nodes_tanh, "busemann_nodes_tanh.png", false)?;

    // create top-clustered mesh
    let mut vertices_top = Vertices::new_2d(400, 200);
    create_busemann_mesh_2d(
        &mut vertices_top,
        2.0,
        WallDistribution::TopClusteredTangent,
        inlet_contour,
        Some(2.0),  // stronger clustering near top
    );
    let mut nodes_top = Nodes::new_2d();
    nodes_top.populate(&vertices_top).expect("failed to populate top-clustered nodes");
    plot_nodes_2d(&nodes_top, "busemann_nodes_top.png", false)?;

    Ok(())
}

enum WallDistribution {
    Uniform,
    HyperbolicTangent,
    TopClusteredTangent,
}

fn inlet_contour(x: f64) -> f64 {
    1.0 - (1.0 / 10.0 * x.powi(2))
}

fn create_busemann_mesh_2d(
    vertices: &mut Vertices<Point2D>,
    lenx: f64,
    wall_distribution: WallDistribution,
    inlet_contour: impl Fn(f64) -> f64,
    beta: Option<f64>,
) {
    let (nx, ny) = vertices.nx_ny();
    let dx: f64 = lenx / (nx as f64 - 1.0);

    match wall_distribution {
        WallDistribution::Uniform => {
            for j in 0..ny {
                for i in 0..nx {
                    let x = i as f64 * dx;
                    let leny = inlet_contour(x);
                    let dy = leny / (ny as f64 - 1.0);
                    let y = j as f64 * dy;
                    
                    let vertex_id = i + j * nx;
                    vertices.add_vertex(Vertex::new_2d(vertex_id, x, y));
                }
            }
        }
        WallDistribution::HyperbolicTangent => {
            let beta = beta.unwrap_or(2.0);

            for j in 0..ny {
                // calculate normalized coordinate eta between 0 and 1
                let eta = j as f64 / (ny - 1) as f64;
                
                for i in 0..nx {
                    let x = i as f64 * dx;
                    let leny = inlet_contour(x);
                    
                    // apply hyperbolic tangent distribution without doubling the height
                    let tanh_term = (beta * (2.0 * eta - 1.0)).tanh() / beta.tanh();
                    let y = leny * (0.5 * (1.0 + tanh_term));  // Scale to [0, leny]

                    let vertex_id = i + j * nx;
                    vertices.add_vertex(Vertex::new_2d(vertex_id, x, y));
                }
            }
        }
        WallDistribution::TopClusteredTangent => {
            let beta = beta.unwrap_or(2.0);

            for j in 0..ny {
                // calculate normalized coordinate eta between 0 and 1
                let eta = j as f64 / (ny - 1) as f64;
                
                for i in 0..nx {
                    let x = i as f64 * dx;
                    let leny = inlet_contour(x);
                    
                    // top-boundary-clustered distribution
                    let tanh_term = (beta * eta).tanh() / beta.tanh();
                    let y = leny * tanh_term;

                    let vertex_id = i + j * nx;
                    vertices.add_vertex(Vertex::new_2d(vertex_id, x, y));
                }
            }
        }
    }
}
