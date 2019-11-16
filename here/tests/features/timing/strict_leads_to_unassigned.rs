use crate::helpers::*;
use crate::json::problem::*;
use crate::json::solution::*;

#[test]
fn can_have_unassigned_jobs_because_of_strict_times() {
    let problem = Problem {
        id: "my_problem".to_string(),
        plan: Plan {
            jobs: vec![
                create_delivery_job_with_times("job1", vec![10., 0.], vec![(0, 10)], 0.),
                create_delivery_job_with_times("job2", vec![20., 0.], vec![(10, 20)], 0.),
                create_delivery_job_with_times("job3", vec![30., 0.], vec![(20, 30)], 0.),
                create_delivery_job_with_times("job4", vec![40., 0.], vec![(30, 40)], 0.),
                create_delivery_job_with_times("job5", vec![50., 0.], vec![(0, 10)], 0.),
            ],
            relations: Option::None,
        },
        fleet: Fleet { types: vec![create_default_vehicle("my_vehicle")] },
    };
    let matrix = create_matrix_from_problem(&problem);

    let solution = solve_with_metaheuristic(problem, vec![matrix]);

    assert_eq!(
        solution,
        Solution {
            problem_id: "my_problem".to_string(),
            statistic: Statistic {
                cost: 170.,
                distance: 80,
                duration: 80,
                times: Timing { driving: 80, serving: 0, waiting: 0, break_time: 0 },
            },
            tours: vec![Tour {
                vehicle_id: "my_vehicle_1".to_string(),
                type_id: "my_vehicle".to_string(),
                stops: vec![
                    create_stop_with_activity(
                        "departure",
                        "departure",
                        (0., 0.),
                        4,
                        ("1970-01-01T00:00:00Z", "1970-01-01T00:00:00Z"),
                    ),
                    create_stop_with_activity(
                        "job1",
                        "delivery",
                        (10., 0.),
                        3,
                        ("1970-01-01T00:00:10Z", "1970-01-01T00:00:10Z"),
                    ),
                    create_stop_with_activity(
                        "job2",
                        "delivery",
                        (20., 0.),
                        2,
                        ("1970-01-01T00:00:20Z", "1970-01-01T00:00:20Z"),
                    ),
                    create_stop_with_activity(
                        "job3",
                        "delivery",
                        (30., 0.),
                        1,
                        ("1970-01-01T00:00:30Z", "1970-01-01T00:00:30Z"),
                    ),
                    create_stop_with_activity(
                        "job4",
                        "delivery",
                        (40., 0.),
                        0,
                        ("1970-01-01T00:00:40Z", "1970-01-01T00:00:40Z"),
                    ),
                    create_stop_with_activity(
                        "arrival",
                        "arrival",
                        (0., 0.),
                        0,
                        ("1970-01-01T00:01:20Z", "1970-01-01T00:01:20Z"),
                    ),
                ],
                statistic: Statistic {
                    cost: 170.,
                    distance: 80,
                    duration: 80,
                    times: Timing { driving: 80, serving: 0, waiting: 0, break_time: 0 },
                },
            }],
            unassigned: vec![UnassignedJob {
                job_id: "job5".to_string(),
                reasons: vec![UnassignedJobReason {
                    code: 2,
                    description: "cannot be visited within time window".to_string()
                }]
            }],
            extras: Extras { performance: vec![] },
        },
    );
}