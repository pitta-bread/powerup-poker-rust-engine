use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::engine::{Action, GameState, PowerUpUse, TurnOutcome};

#[pyclass]
pub struct PyGameState {
    inner: GameState,
}

#[pymethods]
impl PyGameState {
    #[new]
    fn new(player_1_name: &str, player_2_name: &str) -> Self {
        Self {
            inner: GameState::setup_new_game(player_1_name, player_2_name),
        }
    }

    #[getter]
    fn phase(&self) -> String {
        format!("{:?}", self.inner.phase)
    }

    #[getter]
    fn active_player(&self) -> usize {
        self.inner.active_player
    }

    fn start_hand(&mut self) -> PyResult<()> {
        self.inner.start_hand().map_err(PyValueError::new_err)
    }

    #[pyo3(signature = (player_id, action, bet_size=None, use_power_up=false, card_chosen_idx=None))]
    fn step_turn(
        &mut self,
        player_id: usize,
        action: &str,
        bet_size: Option<u8>,
        use_power_up: bool,
        card_chosen_idx: Option<u8>,
    ) -> PyResult<String> {
        let parsed_action = match action {
            "fold" => Action::Fold,
            "check_or_call" => Action::CheckOrCall,
            "bet" => {
                let size = bet_size.ok_or_else(|| {
                    PyValueError::new_err("bet_size is required when action='bet'")
                })?;
                Action::Bet(size)
            }
            _ => {
                return Err(PyValueError::new_err(
                    "unknown action: use fold, check_or_call, or bet",
                ));
            }
        };

        let power_up_use = if use_power_up {
            Some(PowerUpUse { card_chosen_idx })
        } else {
            None
        };

        let outcome = self
            .inner
            .step_turn(player_id, parsed_action, power_up_use)
            .map_err(PyValueError::new_err)?;

        let outcome_str = match outcome {
            TurnOutcome::NextPlayer => "NextPlayer",
            TurnOutcome::NextPhase => "NextPhase",
            TurnOutcome::NextHand => "NextHand",
        };
        Ok(outcome_str.to_string())
    }

    fn advance_phase(&mut self) -> PyResult<()> {
        self.inner.advance_phase().map_err(PyValueError::new_err)
    }

    fn resolve_showdown(&mut self) -> PyResult<String> {
        let result = self
            .inner
            .resolve_showdown()
            .map_err(PyValueError::new_err)?;
        serde_json::to_string(&result).map_err(PyValueError::new_err)
    }

    fn player_view(&self, player_id: usize) -> PyResult<String> {
        let view = self
            .inner
            .player_view(player_id)
            .map_err(PyValueError::new_err)?;
        serde_json::to_string(&view).map_err(PyValueError::new_err)
    }

    fn game_winner(&self) -> Option<usize> {
        self.inner.game_winner()
    }

    fn to_json(&self) -> PyResult<String> {
        self.inner.to_json().map_err(PyValueError::new_err)
    }

    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        let inner = GameState::from_json(json).map_err(PyValueError::new_err)?;
        Ok(Self { inner })
    }
}

#[pymodule]
fn powerup_poker_rust_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGameState>()?;
    Ok(())
}
