use std::{
    cmp::{max, Ordering},
    collections::HashMap,
};

struct MonitoringSystem {
    applications: HashMap<String, Application>,
}

#[derive(Clone)]
struct Application {
    latency: HashMap<String, Vec<i32>>,
    errors: HashMap<String, Vec<i32>>,
}

impl Application {
    fn new() -> Self {
        Application {
            latency: HashMap::new(),
            errors: HashMap::new(),
        }
    }
    fn log_error_code(&mut self, api: String, error_code: i32) {
        match self.errors.get(&api) {
            Some(existing_errors) => {
                let mut updated_errors = existing_errors.clone();
                updated_errors.push(error_code.clone());
                self.errors.insert(api, updated_errors);
            }
            None => {
                self.errors.insert(api, vec![error_code]);
            }
        }
    }

    fn add_latency_measurement(&mut self, api: String, latency_in_mills: i32) {
        match self.latency.get(&api) {
            Some(existing_latency_measurements) => {
                let mut updates_latency_measurements = existing_latency_measurements.clone();
                updates_latency_measurements.push(latency_in_mills.clone());
                self.latency.insert(api, updates_latency_measurements);
            }
            None => {
                self.latency.insert(api, vec![latency_in_mills]);
            }
        }
    }

    fn get_percentile_latency(&self, api_name: String, percentile: i32) -> i32 {
        let mut latency_records = self.latency.get(&api_name).unwrap().clone();
        latency_records.sort();
        let index = max(0, ((percentile * latency_records.len() as i32) / 100) - 1);
        latency_records[index as usize]
    }

    fn get_top_error_codes(&self, api_name: String) -> Vec<i32> {
        let errors = self.errors.get(&api_name).unwrap();
        // Get counts of each error
        let mut error_counts: HashMap<i32, i32> = HashMap::new();
        for error in errors {
            match error_counts.get(error) {
                Some(error_count) => {
                    error_counts.insert(error.clone(), error_count.clone() + 1);
                }
                None => {
                    error_counts.insert(error.clone(), 1);
                }
            }
        }

        // Sort errors by count
        let mut ordered_errors = Vec::new();
        for error in error_counts {
            ordered_errors.push(error)
        }
        ordered_errors.sort_by(|a, b| order_codes(a, b));
        ordered_errors.iter().map(|e| e.0).collect()
    }
}

fn order_codes(a: &(i32, i32), b: &(i32, i32)) -> Ordering {
    if a.1 > b.1 {
        Ordering::Less
    } else if a.1 == b.1 {
        if a.0 > b.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else {
        Ordering::Greater
    }
}

impl MonitoringSystem {
    fn new() -> Self {
        MonitoringSystem {
            applications: HashMap::new(),
        }
    }

    fn log_latency(&mut self, application_name: String, api: String, latency_in_mills: i32) {
        let app = self.applications.get(&application_name);
        let mut updated_application = Application::new();
        match app {
            Some(existing_application) => {
                updated_application = existing_application.clone();
                updated_application.add_latency_measurement(api, latency_in_mills);
            }
            None => updated_application.add_latency_measurement(api, latency_in_mills),
        }
        self.applications
            .insert(application_name, updated_application);
    }

    fn log_error(&mut self, application_name: String, api: String, error_code: i32) {
        let app = self.applications.get(&application_name);
        let mut updated_application = Application::new();
        match app {
            Some(existing_application) => {
                updated_application = existing_application.clone();
                updated_application.log_error_code(api, error_code);
            }
            None => updated_application.log_error_code(api, error_code),
        }
        self.applications
            .insert(application_name, updated_application);
    }

    fn get_percentile_latency(
        &self,
        percentile: i32,
        application_name: String,
        api: String,
    ) -> i32 {
        let app = self.applications.get(&application_name).unwrap();
        app.get_percentile_latency(api, percentile)
    }

    fn get_top_errors(&self, application_name: String, api: String) -> Vec<i32> {
        self.applications
            .get(&application_name)
            .unwrap()
            .get_top_error_codes(api)
    }
}

#[cfg(test)]
mod monitoring_system_tests {
    use super::*;

    #[test]
    fn insert_latency_to_new_app() {
        let mut ms = MonitoringSystem::new();
        let app_name = "security".to_string();
        let api_name = "get-user".to_string();
        let latency1 = 10;
        let latency2 = 1;
        let latency3 = 83;
        let latency4 = 4;
        ms.log_latency(app_name.clone(), api_name.clone(), latency1);
        ms.log_latency(app_name.clone(), api_name.clone(), latency2);
        ms.log_latency(app_name.clone(), api_name.clone(), latency3);
        ms.log_latency(app_name.clone(), api_name.clone(), latency4);
        assert_eq!(
            ms.applications
                .get(&app_name)
                .unwrap()
                .latency
                .get(&api_name)
                .unwrap(),
            &vec![latency1, latency2, latency3, latency4]
        );
        let percentile1 = ms.get_percentile_latency(1, app_name.clone(), api_name.clone());
        assert_eq!(percentile1, 1);
        let percentile50 = ms.get_percentile_latency(50, app_name.clone(), api_name.clone());
        assert_eq!(percentile50, 4);
        let percentile95 = ms.get_percentile_latency(95, app_name, api_name);
        assert_eq!(percentile95, 10)
    }

    #[test]
    fn error_test() {
        let mut ms = MonitoringSystem::new();
        let app_name = "security".to_string();
        let api_name = "get-user".to_string();
        let error1 = 404;
        let error2 = 404;
        let error3 = 503;
        let error4 = 500;
        ms.log_error(app_name.clone(), api_name.clone(), error1);
        ms.log_error(app_name.clone(), api_name.clone(), error2);
        ms.log_error(app_name.clone(), api_name.clone(), error3);
        ms.log_error(app_name.clone(), api_name.clone(), error4);
        assert_eq!(
            ms.get_top_errors(app_name, api_name),
            vec![error1, error4, error3]
        );
    }
}
