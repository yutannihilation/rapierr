use extendr_api::prelude::*;
use rapier2d::{parry::partitioning::IndexedData, prelude::*};

mod result;

fn bouncing_ball_inner() -> Robj {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let collider_bottom = ColliderBuilder::cuboid(100.0, 0.0)
        .translation(vector![-50.0, 0.0])
        .restitution(0.70)
        .build();
    let collider_left = ColliderBuilder::cuboid(100.0, 0.0)
        .rotation(std::f32::consts::PI * 0.5)
        .translation(vector![-0.2, 0.0])
        .restitution(0.70)
        .build();
    collider_set.insert(collider_bottom);
    collider_set.insert(collider_left);

    let pos = &[vector![0.0, 1.0], vector![0.02, 0.8]];

    let handles = pos.map(|v| {
        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic().translation(v).build();
        let collider = ColliderBuilder::ball(0.07)
            .restitution(0.97)
            .restitution_combine_rule(CoefficientCombineRule::Multiply)
            .build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        ball_body_handle
    });

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();
    let physics_hooks = ();
    let event_handler = ();

    const FRAMES: usize = 200;

    let mut frame = extendr_api::Integers::new(FRAMES * pos.len());
    let mut index = extendr_api::Integers::new(FRAMES * pos.len());
    let mut x = extendr_api::Doubles::new(FRAMES * pos.len());
    let mut y = extendr_api::Doubles::new(FRAMES * pos.len());

    let mut i = 0_usize;

    /* Run the game loop, stepping the simulation once per frame. */
    for f in 0..FRAMES {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        for &h in &handles {
            let ball_body = &rigid_body_set[h];
            frame.set_elt(i, (f as i32).into());
            index.set_elt(i, (h.index() as i32).into());
            x.set_elt(i, (ball_body.translation().x as f64).into());
            y.set_elt(i, (ball_body.translation().y as f64).into());

            i += 1;
        }
    }

    result::ResultTibble { frame, index, x, y }
        .try_into()
        .unwrap()
}

/// Return rapier results
/// @export
#[extendr(use_try_from = true)]
fn bouncing_ball() -> Robj {
    bouncing_ball_inner()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rapierr;
    fn bouncing_ball;
}
