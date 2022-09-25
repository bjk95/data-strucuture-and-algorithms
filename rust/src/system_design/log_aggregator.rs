use std::collections::HashMap;

#[derive(Debug)]
struct LogAggregator {
    logs: HashMap<i32, Log>,
    machines: HashMap<i32, Vec<i32>>,
    services: HashMap<i32, Vec<i32>>,
}

#[derive(Debug)]
struct Log {
    log_id: i32,
    machine_id: i32,
    service_id: i32,
    message: String,
}

impl Log {
    fn message_contains(&self, search_string: &String) -> bool {
        self.message.contains(search_string)
    }
}

impl LogAggregator {
    fn new(machines: i32, services: i32) -> Self {
        LogAggregator {
            services: HashMap::new(),
            machines: HashMap::new(),
            logs: HashMap::new(),
        }
    }

    fn push_log(&mut self, log_id: i32, machine_id: i32, service_id: i32, message: String) {
        // Create new log
        let new_log = Log {
            log_id,
            machine_id,
            service_id,
            message,
        };
        // Insert new log
        self.logs.insert(log_id, new_log);

        // Insert log ID into machine
        let existing_machine = self.machines.get(&machine_id);

        if let Some(m) = existing_machine {
            let mut new_machine = m.clone();
            new_machine.push(log_id);
            self.machines.insert(machine_id, new_machine);
        } else {
            self.machines.insert(machine_id, vec![log_id]);
        }

        // Insert log ID into service index
        let existing_service = self.services.get(&service_id);

        if let Some(m) = existing_service {
            let mut new_service = m.clone();
            new_service.push(log_id);
            self.services.insert(service_id, new_service);
        } else {
            self.services.insert(service_id, vec![log_id]);
        }
    }

    fn get_logs_from_machine(&self, machine_id: i32) -> Vec<i32> {
        match self.machines.get(&machine_id) {
            Some(logs) => logs.clone(),
            None => Vec::new(),
        }
    }

    fn get_logs_of_service(&self, service_id: i32) -> Vec<i32> {
        match self.services.get(&service_id) {
            Some(logs) => logs.clone(),
            None => Vec::new(),
        }
    }

    fn search(&self, service_id: i32, search_string: String) -> Vec<String> {
        let r: Vec<String> = self
            .get_logs_of_service(service_id)
            .iter()
            .map(|l| self.logs.get(l).unwrap())
            .filter(|s| s.message_contains(&search_string))
            .map(|l| l.message.clone())
            .collect();
        r
    }
}

#[cfg(test)]
mod log_aggregator_tests {
    use super::*;

    fn test_log() -> Log {
        Log {
            log_id: 1,
            machine_id: 1,
            service_id: 1,
            message: "I'm a test".to_string(),
        }
    }

    #[test]
    fn test_push_log() {
        let test_log = test_log();
        let mut test_aggregator = LogAggregator::new(1, 1);
        test_aggregator.push_log(
            test_log.log_id,
            test_log.machine_id,
            test_log.service_id,
            test_log.message.clone(),
        );
        assert_eq!(
            &test_aggregator.logs.get(&test_log.log_id).unwrap().message,
            &test_log.message
        )
    }

    #[test]
    fn test_get_machine_logs() {
        let mut test_aggregator = LogAggregator::new(2, 1);
        test_aggregator.push_log(1, 1, 1, "I'm a test".to_string());
        test_aggregator.push_log(2, 1, 1, "I'm a test".to_string());
        test_aggregator.push_log(3, 2, 1, "I'm a test".to_string());

        assert_eq!(test_aggregator.get_logs_from_machine(1), vec![1, 2])
    }

    #[test]
    fn test_get_service_logs() {
        let mut test_aggregator = LogAggregator::new(2, 1);
        test_aggregator.push_log(1, 1, 1, "I'm a test".to_string());
        test_aggregator.push_log(2, 1, 1, "I'm a test".to_string());
        test_aggregator.push_log(3, 1, 3, "I'm a test".to_string());
        test_aggregator.push_log(4, 1, 3, "I'm a test".to_string());
        test_aggregator.push_log(5, 1, 4, "I'm a test".to_string());

        assert_eq!(test_aggregator.get_logs_of_service(3), vec![3, 4])
    }

    #[test]
    fn test_search() {
        let mut test_aggregator = LogAggregator::new(2, 1);
        test_aggregator.push_log(1, 1, 1, "I'm a test".to_string());
        test_aggregator.push_log(2, 1, 1, "I'm not a test".to_string());

        assert_eq!(
            test_aggregator.search(1, "not".to_string()),
            vec!["I'm not a test".to_string()]
        )
    }

    #[test]
    fn test_log_message_contains() {
        assert!(test_log().message_contains(&"test".to_string()));
    }
}
