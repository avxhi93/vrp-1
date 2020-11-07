use crate::solver::evolution::*;
use crate::solver::{RefinementContext, Telemetry};
use crate::utils::Timer;

/// A simple evolution algorithm which maintains single population.
pub struct RunSimple {}

impl Default for RunSimple {
    fn default() -> Self {
        Self {}
    }
}

impl EvolutionStrategy for RunSimple {
    fn run(
        &self,
        refinement_ctx: RefinementContext,
        selection: &(dyn Selection + Send + Sync),
        mutation: &(dyn Mutation + Send + Sync),
        termination: &(dyn Termination + Send + Sync),
        telemetry: Telemetry,
    ) -> EvolutionResult {
        let mut refinement_ctx = refinement_ctx;
        let mut telemetry = telemetry;

        while !should_stop(&mut refinement_ctx, termination) {
            let generation_time = Timer::start();

            let parents = selection.select_parents(&refinement_ctx);

            let offspring = mutation.mutate_all(&refinement_ctx, parents);

            let is_improved =
                if should_add_solution(&refinement_ctx) { refinement_ctx.population.add_all(offspring) } else { false };

            telemetry.on_generation(&mut refinement_ctx, generation_time, is_improved);
        }

        telemetry.on_result(&refinement_ctx);

        Ok((refinement_ctx.population, telemetry.get_metrics()))
    }
}