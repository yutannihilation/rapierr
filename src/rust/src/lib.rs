use extendr_api::prelude::*;
use rapier2d::{parry::partitioning::IndexedData, prelude::*};

mod result;

pub struct Rapier2DWorld {
    physics_pipeline: PhysicsPipeline,
    gravity: rapier2d::na::Vector2<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    body_handles: Vec<RigidBodyHandle>,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    // physics_hooks: dyn PhysicsHooks,
    // event_handler: dyn EventHandler,
}

impl Rapier2DWorld {
    fn step(&mut self, n: usize) -> Robj {
        let mut frame = extendr_api::Integers::new(n * self.object_count());
        let mut index = extendr_api::Integers::new(n * self.object_count());
        let mut x = extendr_api::Doubles::new(n * self.object_count());
        let mut y = extendr_api::Doubles::new(n * self.object_count());

        let mut i = 0_usize;

        for f in 0..n {
            self.physics_pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.island_manager,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.bodies,
                &mut self.colliders,
                &mut self.impulse_joints,
                &mut self.multibody_joints,
                &mut self.ccd_solver,
                Some(&mut self.query_pipeline),
                &(),
                &(),
            );

            for &h in &self.body_handles {
                let ball_body = &self.bodies.get(h).unwrap();
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

    fn add_ball(&mut self, x: f32, y: f32, radius: f32, restitution: f32) {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .build();
        let collider = ColliderBuilder::ball(radius)
            .restitution(restitution)
            .restitution_combine_rule(CoefficientCombineRule::Multiply)
            .build();
        let ball_body_handle = self.bodies.insert(rigid_body);
        self.colliders
            .insert_with_parent(collider, ball_body_handle, &mut self.bodies);
        self.body_handles.push(ball_body_handle);
    }

    fn object_count(&self) -> usize {
        self.body_handles.len()
    }
}

fn bouncing_ball_inner() -> Robj {
    let mut colliders = ColliderSet::new();

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
    colliders.insert(collider_bottom);
    colliders.insert(collider_left);

    let pos = vec![vector![0.0, 1.0], vector![0.02, 0.8], vector![0.01, 1.4]];

    let mut world = Rapier2DWorld {
        physics_pipeline: PhysicsPipeline::new(),
        gravity: vector![0.0, -9.81],
        integration_parameters: IntegrationParameters::default(),
        island_manager: IslandManager::new(),
        broad_phase: BroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        bodies: RigidBodySet::new(),
        body_handles: Vec::new(),
        colliders,
        impulse_joints: ImpulseJointSet::new(),
        multibody_joints: MultibodyJointSet::new(),
        ccd_solver: CCDSolver::new(),
        query_pipeline: QueryPipeline::new(),
    };

    world.add_ball(0.0, 1.0, 0.07, 0.97);
    world.add_ball(0.02, 0.8, 0.07, 0.97);
    world.add_ball(0.01, 1.2, 0.07, 0.97);

    const FRAMES: usize = 200;

    world.step(FRAMES)
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
