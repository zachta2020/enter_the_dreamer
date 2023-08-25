use bevy::prelude::*;

pub struct CounterPlugin;
impl Plugin for CounterPlugin {
        fn build(&self, app: &mut App) {
        app.insert_resource(UpdateTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(TimerCounter { i: 0 })
            .add_systems(Update, timer_print);
    }
}

//Resources
#[derive(Resource)]
struct UpdateTimer(Timer);

#[derive(Resource)]
struct TimerCounter {
    i: u32,
}

impl Default for TimerCounter {
    fn default() -> Self {
        TimerCounter { i: 0 }
    }
}

fn timer_print(
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
    mut counter: ResMut<TimerCounter>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Counter: {}", counter.i);
        counter.i += 1;
        if counter.i == 10 {
            counter.i = 0;
        }
    }
}