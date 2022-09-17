struct DCLoadBalancer {
    machines: Vec<Machine>
}

#[derive(Clone)]
struct Machine {
    id: i32,
    capacity: i32,
    applications: Vec<Application>
}

impl Machine {
    fn get_free_capacity(&self) -> i32 {
        let load_used = self.applications
            .iter()
            .map(|a| a.load)
            .reduce(|a,b| a+b).unwrap_or(0);

        self.capacity - load_used
    }

    fn add_application_to_machine(&mut self, app: Application){
        self.applications.push(app);
    }
}

#[derive(Clone)]
struct Application {
    id: i32,
    load: i32
}


impl DCLoadBalancer {

    fn new() -> Self {
        DCLoadBalancer { 
            machines: Vec::new()
         }
    }
    
    fn add_machine(&mut self, machine_id: i32, capacity: i32) {
        let new_machine = Machine{
            id: machine_id,
            capacity: capacity,
            applications: Vec::new()
        };
        self.machines.push(new_machine);
    }
    
    fn remove_machine(&mut self, machine_id: i32) {
        // Find machine to be removed
        let machine_to_be_removed_index = self.machines.iter().position(|m| m.id == machine_id).unwrap();
        let machine_to_be_removed = self.machines[machine_to_be_removed_index].clone();

        // Reallocate any apps
        for app in machine_to_be_removed.applications{
            self.add_application(app.id, app.load);
        }

        // Remove machine from cluster
        self.machines.remove(machine_to_be_removed_index);
    }
    
    fn add_application(&mut self, app_id: i32, load_use: i32) -> i32 {
        // Find least utilized machine
        let mut least_utilized_machine_option: Option<Machine> = None;
        let mut least_utilized_machine_index = 0;
        for (index, machine) in self.machines.iter().enumerate() {
            match &least_utilized_machine_option {
                Some(lum) => {
                    if &lum.get_free_capacity() < &machine.get_free_capacity() {
                        least_utilized_machine_option = Some(machine.clone());
                        least_utilized_machine_index = index;
                    }
                },
                None => {
                    least_utilized_machine_option = Some(machine.clone());
                },
            }
        }

        // Check if machine can handle application
       let mut least_utilized_machine: Machine; 
        match least_utilized_machine_option {
            Some(m) => {
                if m.get_free_capacity() < load_use {
                    return -1
                } else {
                    least_utilized_machine = m;
                }
            },
            None => {
                return -1
            },
        }

        // Add application to least used machine
        let new_application = Application{
            id: app_id,
            load: load_use
        };
        least_utilized_machine.add_application_to_machine(new_application);
        let least_utilized_machine_id = least_utilized_machine.id;
        self.machines[least_utilized_machine_index] = least_utilized_machine.clone();


        least_utilized_machine.id
    }
    
    fn stop_application(&mut self, app_id: i32) {

        // Get ID of machine running application
        let mut application_machine_id = None;
        let mut application_machine_index = 0;
        for  (index, machine) in self.machines.iter().enumerate() {
            let application_ids: Vec<i32> = machine.applications.iter().map(|a| a.id).collect();
            
            if application_ids.contains(&app_id) {
                application_machine_id = Some(machine.id);
                application_machine_index = index;
                break;
            }
        }

        // Stop application if running
        match application_machine_id {
            Some(_) => {
                let mut application_machine = self.machines[application_machine_index].clone();
                let application_index = application_machine.applications.iter().position(|a| a.id == app_id);
                application_machine.applications.remove(application_index.unwrap());

                self.machines[application_machine_index] = application_machine;
            },
            None => return,
        }

    }
    
    fn get_applications(&self, machine_id: i32) -> Vec<i32> {
        let machine_index = self.machines.iter().position(|m| m.id == machine_id);
        let mut application_ids: Vec<i32> = Vec::new();
        match machine_index{
            Some(m) => {
                let machine = self.machines[m].clone();
                application_ids.append(&mut machine.applications.iter().map(|a| a.id).collect())
            },
            None => {},
        }
        application_ids
    }
}



#[cfg(test)]
mod load_balancer_tests{
    use super::*;

    #[test]
    fn create_load_balancer(){
        let mut lb = DCLoadBalancer::new();
        lb.add_machine(1,1);
        lb.add_machine(2,10);
        lb.add_machine(3,10);
        lb.add_machine(4,15);
        let app_1_machine = lb.add_application(1, 3);
        assert_eq!(app_1_machine, 4);
        let app_2_machine = lb.add_application(2, 11);
        assert_eq!(app_2_machine, 4);
        let app_3_machine = lb.add_application(3,6);
        assert_eq!(app_3_machine, 2);
        let app_4_machine = lb.add_application(4, 5);
        assert_eq!(app_4_machine, 3);
        let machine_2_applications = lb.get_applications(2);
        assert_eq!(machine_2_applications, vec![3]);
        lb.add_machine(5, 10);
        let app_5_machine = lb.add_application(5, 5);
        assert_eq!(app_5_machine,5);
        lb.stop_application(3);
        let app_6_machine = lb.add_application(6, 5);
        assert_eq!(app_6_machine, 2);
        let machine_4_application = lb.get_applications(4);
        assert_eq!(machine_4_application, vec![1,2]);
        lb.remove_machine(4);
        let machine_2_applications = lb.get_applications(2);
        assert_eq!(machine_2_applications, vec![6])
    }
}