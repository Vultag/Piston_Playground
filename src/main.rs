

use bevy::math::vec3;

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;


fn main() 
{
    
    App::new() 
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Piston Playground".into(),
                    resolution: (1600., 900.).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_state::<SimulationState>()
        .add_systems(Startup,spawn_camera)
        .add_systems(Startup,spawn_player)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup,setup_physics)
        .add_systems(Update,toggle_simulation)
        .add_systems(Update,keyboard_input_system)
        .run();

}



#[derive(Component)]
pub struct Player{

	d_piston	: Option<Entity>
	, u_piston	: Option<Entity>
	, r_piston	: Option<Entity>
	, l_piston	: Option<Entity>

}
#[derive(Component)]
pub struct Piston{}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimulationState {

    Running,
    #[default]
    Paused,
}

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if **simulation_state == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
            rapier_configuration.physics_pipeline_active = false;
        }
        if **simulation_state == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
            println!("Simulation Running.");
            rapier_configuration.physics_pipeline_active = true;
        }
    }
}


pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    //mut player_query: Query<&mut Transform, With<Player>>,
    //mut velocity : Query<&mut Velocity, With<Player>>,
    //mut ext_forces: Query<&mut ExternalForce, With<Player>>,

    //mut piston_d: Query<&mut ImpulseJoint, With<PistonD>>,
    //mut piston_d_query: Query<&mut ImpulseJoint,With<PistonD>>,
    //mut piston_u_query: Query<&mut ImpulseJoint,With<PistonU>>,
    //mut piston_r_query: Query<&mut ImpulseJoint,With<PistonR>>,
    //mut piston_l_query: Query<&mut ImpulseJoint,With<PistonL>>,

    mut query: Query<&mut ImpulseJoint,With<Piston>>,
    player :Query<&Player>


    //time: Res<Time>,
) {

    let mut piston_d = query.get_mut(player.single().d_piston.unwrap()).unwrap();


        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {

            piston_d.data.as_prismatic_mut().unwrap().set_motor_velocity(-300.0, 50.0);

        }
        else {

            
            piston_d.data.as_prismatic_mut().unwrap().set_motor_velocity(300.0, 50.0);

        }
        
    let mut piston_u = query.get_mut(player.single().u_piston.unwrap()).unwrap();


    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Z) {


        piston_u.data.as_prismatic_mut().unwrap().set_motor_velocity(300.0, 50.0);

    }
    else {

        
        piston_u.data.as_prismatic_mut().unwrap().set_motor_velocity(-300.0, 50.0);

    }



    let mut piston_r = query.get_mut(player.single().r_piston.unwrap()).unwrap();

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {


        piston_r.data.as_prismatic_mut().unwrap().set_motor_velocity(300.0, 50.0);

    }
    else {

        
        piston_r.data.as_prismatic_mut().unwrap().set_motor_velocity(-300.0, 50.0);

    }


    let mut piston_l = query.get_mut(player.single().l_piston.unwrap()).unwrap();


    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Q) {

    
        piston_l.data.as_prismatic_mut().unwrap().set_motor_velocity(-300.0, 50.0);

    }
    else {

        
        piston_l.data.as_prismatic_mut().unwrap().set_motor_velocity(300.0, 50.0);

    }


}


fn spawn_camera
(
    mut commands: Commands,
    asset_server : Res<AssetServer>,

) 
    {

    commands.spawn(Camera2dBundle::default());
        

       // Text with one section
       commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Z : piston up\nS : piston down\nD : piston right\nQ : piston left",

            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("Fonts/Roboto-Regular.ttf"),
                font_size: 60.0,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(3.0),
            left: Val::Px(3.0),
            ..default()
        }),
    ));

    commands.spawn((
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            " Space : pause/resume simulation",

            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("Fonts/Roboto-Regular.ttf"),
                font_size: 60.0,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Right)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(3.0),
            right: Val::Px(3.0),
            ..default()
        }),
    ));

    commands.spawn((
        SpriteBundle{
            transform: Transform::from_xyz(0.0, 150.0, -1.0).with_scale(vec3(1.,1., 1.)),
            texture: asset_server.load("Sprites/title.png"),
            ..default()
            
        },
    ));


}

fn setup_physics(
    mut commands: Commands,

) {


    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 300.0, 0.0)));
    commands
        .spawn(Collider::cuboid(500.0, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)));    
    commands
        .spawn(Collider::cuboid(20.0, 280.0))
        .insert(TransformBundle::from(Transform::from_xyz(-500.0, 0.0, 0.0)));
    commands
        .spawn(Collider::cuboid(20.0, 280.0))
        .insert(TransformBundle::from(Transform::from_xyz(500.0, 0.0, 0.0)));
    commands
        .spawn(Collider::cuboid(50.0, 60.0))
        .insert(TransformBundle::from(Transform::from_xyz(-430.0, -220.0, 0.0)));
    commands
        .spawn(Collider::cuboid(50.0, 60.0))
        .insert(TransformBundle::from(Transform::from_xyz(430.0, -220.0, 0.0)));
    commands
        .spawn(Collider::ball(80.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -20.0, 0.0)));
    commands
        .spawn(Collider::ball(60.0))
        .insert(TransformBundle::from(Transform::from_xyz(280.0, 20.0, 0.0)));
    commands
        .spawn(Collider::ball(60.0))
        .insert(TransformBundle::from(Transform::from_xyz(-280.0, 20.0, 0.0)));



}




pub fn spawn_player
    (
        mut commands: Commands,
        asset_server : Res<AssetServer>,
        mut rapier_configuration: ResMut<RapierConfiguration>,
    ) 
    {

        let play_pos = Transform::from_xyz(0.0, 200.0, 0.0);

        /* Create the Player body. */
        let phy_body = commands
            .spawn(RigidBody::Dynamic)
            .insert(Collider::cuboid(40.0,40.0))
            .insert(Restitution::coefficient(0.7))
            .insert(SpatialBundle::from(play_pos))
            .insert(Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0})
            .insert(ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            })
            .id();
        let visual_body = commands
            .spawn((
            SpriteBundle{
                transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(vec3(0.5,0.5, 0.5)),
                //transform: Transform::from_xyz(window.width()*0.5, window.height()*0.5, 0.0).with_scale(vec3(0.1,0.1, 0.1)),
                //transform: Transform::from_scale(Vec3 { x: (0.1), y: (0.1), z: (0.1) }),
                texture: asset_server.load("Sprites/perso.png"),
                ..default()
                
            },
        )).id();


        //PISTON DOWN
        let joint_d = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(0.0, 80.0))
        .limits([-20.0, 80.0])
        .motor_velocity(-300.0, 50.0);
    let d_piston = commands.spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(40.0,10.0))
        .insert(GlobalTransform::from_translation(play_pos.translation + Vec3 { x: 0.0, y: -50.0, z: 0.0 }))
        .insert(ImpulseJoint::new(phy_body, joint_d))
        .insert(Piston{}).id();

        //PISTON UP
        let joint_u = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(0.0, 0.0))
        .limits([-20.0, 100.0])
        .motor_velocity(300.0, 50.0);
    let u_piston =  commands.spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(40.0,10.0))
        .insert(GlobalTransform::from_translation(play_pos.translation + Vec3 { x: 0.0, y: 50.0, z: 0.0 }))
        .insert(ImpulseJoint::new(phy_body, joint_u))
        .insert(Piston{}).id();

        //PISTON RIGHT
            let joint_r = PrismaticJointBuilder::new(Vec2::X)
            .local_anchor1(Vec2::new(0.0, 0.0))
            .local_anchor2(Vec2::new(0.0, 0.0))
            .limits([-20.0, 100.0])
            .motor_velocity(300.0, 50.0);
     let r_piston = commands.spawn(RigidBody::Dynamic)
            .insert(Collider::cuboid(10.0,40.0))
            .insert(GlobalTransform::from_translation(play_pos.translation + Vec3 { x: 50.0, y: 0.0, z: 0.0 }))
            .insert(ImpulseJoint::new(phy_body, joint_r))
            .insert(Piston{}).id();

       //PISTON LEFT
        let joint_l = PrismaticJointBuilder::new(Vec2::X)
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(80.0, 0.0))
        .limits([-20.0, 100.0])
        .motor_velocity(-300.0, 50.0);
    let l_piston = commands.spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0,40.0))
        .insert(GlobalTransform::from_translation(play_pos.translation + Vec3 { x: -50.0, y: 0.0, z: 0.0 }))
        .insert(ImpulseJoint::new(phy_body, joint_l))
        .insert(Piston{}).id();
    

commands.entity(phy_body).insert(Player{d_piston: Some(d_piston) , u_piston:Some(u_piston), r_piston: Some(r_piston), l_piston: Some(l_piston) });


        commands.entity(phy_body).push_children(&[visual_body]);


        rapier_configuration.physics_pipeline_active = false;
    
    }


