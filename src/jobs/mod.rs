use actix_web::web;
use job::{generate_event, stop_non_recurring_event};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::AppState;

pub mod job;

pub async fn launchjobs(state: web::Data<AppState>) {
    let is_enabled = state
        .config
        .get::<bool>("cron.enable_cron")
        .unwrap_or(false);

    if !is_enabled {
        println!("Cron job not started: ENABLE_CRON != true");
        return;
    }

    println!("Starting cron jobs...");

    let scheduler = match JobScheduler::new().await {
        Ok(sche) => sche,
        Err(e) => {
            eprintln!("Failed to create scheduler: {:?}", e);
            return;
        }
    };

    let jobs = vec![
        Job::new_async("0 0 * * *", {
            let state = state.clone();
            move |_uuid, _l| {
                let state = state.clone();
                Box::pin(async move {
                    println!("Running event generation job at: {}", chrono::Utc::now());
                    if let Err(e) = generate_event(&state).await {
                        eprintln!("Event generation failed: {e:?}");
                    }
                })
            }
        }),
        Job::new_async("0 0 * * *", {
            let state = state.clone();
            move |_uuid, _l| {
                let state = state.clone();
                Box::pin(async move {
                    println!("Running event ending job at: {}", chrono::Utc::now());
                    if let Err(e) = stop_non_recurring_event(&state).await {
                        eprintln!("Event ending failed: {e:?}");
                    }
                })
            }
        }),
    ];

    for (index, job_result) in jobs.into_iter().enumerate() {
        match job_result {
            Ok(job) => {
                if let Err(e) = scheduler.add(job).await {
                    eprintln!("Failed to add job {} to scheduler: {:?}", index + 1, e);
                    return;
                }
            }
            Err(e) => {
                eprintln!("Failed to create cron job {}: {:?}", index + 1, e);
                return;
            }
        }
    }

    if let Err(e) = scheduler.start().await {
        eprintln!("Failed to start scheduler: {:?}", e);
        return;
    }
}

// THIS IS FOR RUNNING A SINGLE JOB
// pub async fn launchjobs(state: web::Data<AppState>) {
//     let is_enabled = state
//         .config
//         .get::<bool>("cron.enable_cron")
//         .unwrap_or(false);

//     if !is_enabled {
//         println!("Cron job not started: ENABLE_CRON != true");
//         return;
//     }

//     println!("Starting cron jobs...");

//     let scheduler = match JobScheduler::new().await {
//         Ok(sche) => sche,
//         Err(e) => {
//             eprintln!("Failed to create scheduler: {:?}", e);
//             return;
//         }
//     };

//     let state_clone = state.clone();

//     let job = match Job::new_async("0 0 * * *", move |_uuid, _l| {

//         let state = state_clone.clone();

//         Box::pin(async move {
//             println!("Running scheduled job at: {}", chrono::Utc::now());

//             if let Err(e) = generate_event(&state).await {
//                 eprintln!("Event generation failed: {e:?}");
//             }
//         })
//     }) {
//         Ok(job) => job,
//         Err(e) => {
//             eprintln!("Failed to create cron job: {:?}", e);
//             return;
//         }
//     };

//     if let Err(e) = scheduler.add(job).await {
//         eprintln!("Failed to add job to scheduler: {:?}", e);
//         return;
//     }

//     if let Err(e) = scheduler.start().await {
//         eprintln!("Failed to start scheduler: {:?}", e);
//         return;
//     }
// }

//use this method to multiple function
// rt.block_on(async move {
//     let state1 = state_clone.clone();
//     let state2 = state_clone.clone();

//     Spawn task 1
//     let job1 = task::spawn_local(async move {
//         loop {
//             if let Err(e) = generate_event(&state1).await {
//                 eprintln!("generate_event failed: {e:?}");
//             }
//             tokio::time::sleep(Duration::from_secs(3600)).await; // 1 hour
//         }
//     });

//     Spawn task 2
//     let job2 = task::spawn_local(async move {
//         loop {
//             if let Err(e) = another_job(&state2).await {
//                 eprintln!("another_job failed: {e:?}");
//             }
//             tokio::time::sleep(Duration::from_secs(600)).await; // 10 mins
//         }
//     });

//     Await both
//     futures::future::join(job1, job2).await;
// });
