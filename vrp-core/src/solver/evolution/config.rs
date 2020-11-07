use crate::construction::heuristics::InsertionContext;
use crate::construction::Quota;
use crate::models::Problem;
use crate::solver::evolution::{EvolutionStrategy, RunSimple};
use crate::solver::mutation::*;
use crate::solver::selection::{NaiveSelection, Selection};
use crate::solver::telemetry::Telemetry;
use crate::solver::termination::*;
use crate::solver::TelemetryMode;
use crate::utils::{get_cpus, DefaultRandom, Random};
use std::sync::Arc;

/// A configuration which controls evolution execution.
pub struct EvolutionConfig {
    /// An original problem.
    pub problem: Arc<Problem>,

    /// A evolution operators config.
    pub operators: OperatorConfig,

    /// An evolution strategy.
    pub strategy: Arc<dyn EvolutionStrategy + Send + Sync>,

    /// A quota for evolution execution.
    pub quota: Option<Arc<dyn Quota + Send + Sync>>,

    /// A population configuration
    pub population: PopulationConfig,

    /// Random generator.
    pub random: Arc<dyn Random + Send + Sync>,

    /// A telemetry to be used.
    pub telemetry: Telemetry,
}

/// Contains population specific properties.
pub struct PopulationConfig {
    /// An initial solution config.
    pub initial: InitialConfig,

    /// Max population size.
    pub max_size: usize,
}

#[derive(Clone)]
pub struct OperatorConfig {
    /// A selection defines parents to be selected on each generation.
    pub selection: Arc<dyn Selection + Send + Sync>,

    /// A mutation applied to population.
    pub mutation: Arc<dyn Mutation + Send + Sync>,

    /// A termination defines when evolution should stop.
    pub termination: Arc<dyn Termination + Send + Sync>,
}

/// An initial solutions configuration.
pub struct InitialConfig {
    /// Initial size of population to be generated.
    pub size: usize,

    /// Create methods to produce initial individuals.
    pub methods: Vec<(Box<dyn Recreate + Send + Sync>, usize)>,

    /// Initial individuals in population.
    pub individuals: Vec<InsertionContext>,
}

impl EvolutionConfig {
    /// Creates a new instance of `EvolutionConfig` using default settings.
    pub fn new(problem: Arc<Problem>) -> Self {
        Self {
            problem: problem.clone(),
            operators: OperatorConfig {
                selection: Arc::new(NaiveSelection::new(get_cpus())),
                mutation: Arc::new(CompositeMutation::new(vec![(
                    vec![
                        (Arc::new(LocalSearch::new(Box::new(CompositeLocalOperator::default()))), 0.05),
                        (Arc::new(RuinAndRecreate::new_from_problem(problem)), 1.),
                        (Arc::new(LocalSearch::new(Box::new(CompositeLocalOperator::default()))), 0.01),
                    ],
                    1,
                )])),
                termination: Arc::new(CompositeTermination::new(vec![
                    Box::new(MaxTime::new(300.)),
                    Box::new(MaxGeneration::new(3000)),
                ])),
            },
            strategy: Arc::new(RunSimple::default()),
            quota: None,
            random: Arc::new(DefaultRandom::default()),
            telemetry: Telemetry::new(TelemetryMode::None),
            population: PopulationConfig {
                max_size: 2,
                initial: InitialConfig {
                    size: 1,
                    methods: vec![(Box::new(RecreateWithCheapest::default()), 10)],
                    individuals: vec![],
                },
            },
        }
    }
}