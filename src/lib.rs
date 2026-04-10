/*!
# cuda-self-model

An agent's model of itself.

Metacognition — knowing what you know and what you don't. An agent
without a self-model can't say "I can't do that" or "I'm bad at this
but improving." It's the difference between a tool and a being.

- Capability inventory (what can I do?)
- Limitation awareness (what can't I do?)
- State tracking (energy, confidence, load)
- Self-assessment accuracy (does my self-image match reality?)
- Growth trajectory (am I getting better?)
- Calibration (adjusting self-model based on evidence)
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A capability the agent believes it has
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub category: String,
    pub self_assessed: f64,     // how good agent thinks it is [0, 1]
    pub actual_performance: f64, // what evidence shows [0, 1]
    pub usage_count: u32,
    pub last_used: u64,
}

impl Capability {
    pub fn calibration(&self) -> f64 {
        // How well does self-assessment match reality?
        // 1.0 = perfectly calibrated, 0.0 = completely wrong
        1.0 - (self.self_assessed - self.actual_performance).abs()
    }

    pub fn is_calibrated(&self, threshold: f64) -> bool { self.calibration() >= threshold }

    pub fn overconfident(&self) -> bool { self.self_assessed > self.actual_performance + 0.15 }
    pub fn underconfident(&self) -> bool { self.actual_performance > self.self_assessed + 0.15 }
}

/// A known limitation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Limitation {
    pub name: String,
    pub description: String,
    pub severity: f64,        // 0-1, how limiting
    pub workaround: Option<String>,
    pub discovered: u64,      // when this was discovered
}

/// Internal state snapshot
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InternalState {
    pub energy: f64,          // 0-1
    pub confidence: f64,      // 0-1
    pub cognitive_load: f64,  // 0-1
    pub stress: f64,          // 0-1
    pub boredom: f64,         // 0-1
    pub timestamp: u64,
}

impl InternalState {
    pub fn new() -> Self {
        InternalState { energy: 1.0, confidence: 0.5, cognitive_load: 0.0, stress: 0.0, boredom: 0.0, timestamp: now() }
    }

    /// Overall readiness score
    pub fn readiness(&self) -> f64 {
        (self.energy * 0.4 + self.confidence * 0.3 + (1.0 - self.cognitive_load) * 0.2 + (1.0 - self.stress) * 0.1).clamp(0.0, 1.0)
    }

    /// Am I in a good state for complex tasks?
    pub fn can_do_complex_work(&self) -> bool {
        self.energy > 0.3 && self.cognitive_load < 0.7 && self.stress < 0.5
    }
}

/// Growth record — tracking improvement over time
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrowthRecord {
    pub measurements: Vec<(u64, f64)>, // (timestamp, performance)
    pub trend: GrowthTrend,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrowthTrend {
    Improving,
    Stable,
    Declining,
    InsufficientData,
}

impl GrowthRecord {
    pub fn record(&mut self, timestamp: u64, performance: f64) {
        self.measurements.push((timestamp, performance));
        if self.measurements.len() > 100 { self.measurements.remove(0); }
        self.update_trend();
    }

    fn update_trend(&mut self) {
        if self.measurements.len() < 5 { self.trend = GrowthTrend::InsufficientData; return; }
        let first_half: f64 = self.measurements[..self.measurements.len() / 2].iter().map(|(_, v)| v).sum::<f64>() / (self.measurements.len() / 2) as f64;
        let second_half: f64 = self.measurements[self.measurements.len() / 2..].iter().map(|(_, v)| v).sum::<f64>() / (self.measurements.len() - self.measurements.len() / 2) as f64;
        let diff = second_half - first_half;
        self.trend = if diff > 0.05 { GrowthTrend::Improving } else if diff < -0.05 { GrowthTrend::Declining } else { GrowthTrend::Stable };
    }

    pub fn current_level(&self) -> f64 {
        self.measurements.last().map(|(_, v)| *v).unwrap_or(0.5)
    }

    pub fn improvement_rate(&self) -> f64 {
        if self.measurements.len() < 2 { return 0.0; }
        let first = self.measurements.first().unwrap().1;
        let last = self.measurements.last().unwrap().1;
        let time_span = self.measurements.last().unwrap().0.saturating_sub(self.measurements.first().unwrap().0) as f64;
        if time_span < 1.0 { return 0.0; }
        (last - first) / time_span * 1000.0 // per second
    }
}

/// The self-model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelfModel {
    pub agent_id: String,
    pub capabilities: HashMap<String, Capability>,
    pub limitations: Vec<Limitation>,
    pub state: InternalState,
    pub growth: HashMap<String, GrowthRecord>,
    pub calibration_threshold: f64,
    pub self_awareness: f64,    // how accurate is this model? improves over time
}

impl SelfModel {
    pub fn new(agent_id: &str) -> Self {
        SelfModel { agent_id: agent_id.to_string(), capabilities: HashMap::new(), limitations: vec![], state: InternalState::new(), growth: HashMap::new(), calibration_threshold: 0.8, self_awareness: 0.3 }
    }

    /// Add a capability
    pub fn add_capability(&mut self, cap: Capability) {
        self.capabilities.insert(cap.name.clone(), cap);
    }

    /// Record actual performance for a capability
    pub fn record_performance(&mut self, capability: &str, performance: f64) {
        let actual = performance.clamp(0.0, 1.0);
        if let Some(cap) = self.capabilities.get_mut(capability) {
            cap.actual_performance = cap.actual_performance * 0.7 + actual * 0.3; // EMA
            cap.usage_count += 1;
            cap.last_used = now();

            // Calibrate self-assessment toward reality
            let adjustment = (actual - cap.self_assessed) * 0.1;
            cap.self_assessed = (cap.self_assessed + adjustment).clamp(0.0, 1.0);
        }

        // Record growth
        let record = self.growth.entry(capability.to_string()).or_insert_with(|| GrowthRecord { measurements: vec![], trend: GrowthTrend::InsufficientData });
        record.record(now(), actual);

        // Update self-awareness based on calibration accuracy
        let calibrations: Vec<f64> = self.capabilities.values().map(|c| c.calibration()).collect();
        if !calibrations.is_empty() {
            self.self_awareness = calibrations.iter().sum::<f64>() / calibrations.len() as f64;
        }
    }

    /// Can I do this task?
    pub fn can_do(&self, capability: &str, min_proficiency: f64) -> CapabilityAssessment {
        let cap = match self.capabilities.get(capability) {
            Some(c) => c,
            None => return CapabilityAssessment { capable: false, confidence: 0.0, reason: "unknown capability".into() },
        };
        if cap.self_assessed < min_proficiency {
            return CapabilityAssessment { capable: false, confidence: cap.self_assessed, reason: format!("below threshold ({:.2} < {:.2})", cap.self_assessed, min_proficiency) };
        }
        if cap.overconfident() {
            return CapabilityAssessment { capable: true, confidence: cap.self_assessed * 0.7, reason: "overconfident — may fail".into() };
        }
        CapabilityAssessment { capable: true, confidence: cap.self_assessed, reason: "ready".into() }
    }

    /// Add limitation
    pub fn add_limitation(&mut self, lim: Limitation) { self.limitations.push(lim); }

    /// Known limitations that apply to a task
    pub fn applicable_limitations(&self, tags: &[&str]) -> Vec<&Limitation> {
        self.limitations.iter().filter(|l| tags.iter().any(|t| l.name.contains(t) || l.description.contains(t))).collect()
    }

    /// Uncalibrated capabilities that need attention
    pub fn uncalibrated_capabilities(&self) -> Vec<&Capability> {
        self.capabilities.values().filter(|c| c.usage_count >= 3 && !c.is_calibrated(self.calibration_threshold)).collect()
    }

    /// Update internal state
    pub fn update_state(&mut self, energy: f64, confidence: f64, cognitive_load: f64, stress: f64) {
        self.state.energy = energy;
        self.state.confidence = confidence;
        self.state.cognitive_load = cognitive_load;
        self.state.stress = stress;
        self.state.timestamp = now();
    }

    /// Overall self-summary
    pub fn summary(&self) -> String {
        let total_caps = self.capabilities.len();
        let calibrated = self.capabilities.values().filter(|c| c.is_calibrated(self.calibration_threshold)).count();
        let improving = self.growth.values().filter(|g| g.trend == GrowthTrend::Improving).count();
        format!("SelfModel[{}]: {} capabilities ({} calibrated), {} limitations, readiness={:.2}, self_awareness={:.2}, {} improving",
            self.agent_id, total_caps, calibrated, self.limitations.len(), self.state.readiness(), self.self_awareness, improving)
    }
}

#[derive(Clone, Debug)]
pub struct CapabilityAssessment {
    pub capable: bool,
    pub confidence: f64,
    pub reason: String,
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_calibration() {
        let mut cap = Capability { name: "nav".into(), category: "move".into(), self_assessed: 0.9, actual_performance: 0.5, usage_count: 5, last_used: 0 };
        assert!(!cap.is_calibrated(0.8));
    }

    #[test]
    fn test_overconfident() {
        let cap = Capability { name: "x".into(), category: "y".into(), self_assessed: 0.9, actual_performance: 0.6, usage_count: 1, last_used: 0 };
        assert!(cap.overconfident());
    }

    #[test]
    fn test_state_readiness() {
        let state = InternalState { energy: 1.0, confidence: 1.0, cognitive_load: 0.0, stress: 0.0, boredom: 0.0, timestamp: 0 };
        assert!(state.readiness() > 0.8);
    }

    #[test]
    fn test_cannot_do_complex() {
        let state = InternalState { energy: 0.1, confidence: 0.5, cognitive_load: 0.9, stress: 0.8, boredom: 0.0, timestamp: 0 };
        assert!(!state.can_do_complex_work());
    }

    #[test]
    fn test_growth_record() {
        let mut gr = GrowthRecord { measurements: vec![], trend: GrowthTrend::InsufficientData };
        for i in 0..10 { gr.record(i as u64 * 1000, 0.5 + i as f64 * 0.05); }
        assert_eq!(gr.trend, GrowthTrend::Improving);
    }

    #[test]
    fn test_self_model_can_do() {
        let mut sm = SelfModel::new("agent1");
        sm.add_capability(Capability { name: "navigate".into(), category: "move".into(), self_assessed: 0.8, actual_performance: 0.8, usage_count: 5, last_used: 0 });
        let assessment = sm.can_do("navigate", 0.5);
        assert!(assessment.capable);
    }

    #[test]
    fn test_self_model_unknown() {
        let sm = SelfModel::new("agent1");
        let assessment = sm.can_do("fly", 0.5);
        assert!(!assessment.capable);
    }

    #[test]
    fn test_record_performance_calibrates() {
        let mut sm = SelfModel::new("agent1");
        sm.add_capability(Capability { name: "fight".into(), category: "combat".into(), self_assessed: 0.9, actual_performance: 0.5, usage_count: 0, last_used: 0 });
        // After many bad performances, self-assessed should decrease
        for _ in 0..10 { sm.record_performance("fight", 0.3); }
        let cap = sm.capabilities.get("fight").unwrap();
        assert!(cap.self_assessed < 0.9); // calibrated down
    }

    #[test]
    fn test_limitations() {
        let mut sm = SelfModel::new("agent1");
        sm.add_limitation(Limitation { name: "no_flying".into(), description: "Cannot fly".into(), severity: 1.0, workaround: Some("use navigation instead".into()), discovered: 0 });
        let lims = sm.applicable_limitations(&["flying"]);
        assert_eq!(lims.len(), 1);
    }

    #[test]
    fn test_uncalibrated() {
        let mut sm = SelfModel::new("agent1");
        sm.add_capability(Capability { name: "a".into(), category: "x".into(), self_assessed: 0.9, actual_performance: 0.5, usage_count: 5, last_used: 0 });
        assert!(!sm.uncalibrated_capabilities().is_empty());
    }

    #[test]
    fn test_summary() {
        let sm = SelfModel::new("agent1");
        let s = sm.summary();
        assert!(s.contains("0 capabilities"));
    }

    #[test]
    fn test_self_awareness_improves() {
        let mut sm = SelfModel::new("agent1");
        sm.add_capability(Capability { name: "a".into(), category: "x".into(), self_assessed: 0.7, actual_performance: 0.7, usage_count: 0, last_used: 0 });
        for _ in 0..5 { sm.record_performance("a", 0.7); }
        assert!(sm.self_awareness > 0.5);
    }
}
