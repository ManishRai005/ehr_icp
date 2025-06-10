use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::storage;
use ic_cdk_macros::{init, update, query};
use candid::{candid_method, Decode, Encode};

mod types;
use types::*;


mod types;
use types::{Doctor, MedicalRecord, Patient};

// 🔐 Global in-memory data storage
thread_local! {
    static PATIENTS: RefCell<HashMap<String, Patient>> = RefCell::new(HashMap::new());
    static DOCTORS: RefCell<HashMap<String, Doctor>> = RefCell::new(HashMap::new());
}

#[init]
fn init() {
    ic_cdk::println!("EHR ICP backend initialized");
}

#[update(name = "addDoctor")]
#[candid_method(update)]
fn add_doctor(id: String, name: String, specialization: String) {
    let doctor = Doctor {
        id: id.clone(),
        name,
        specialization,
        principal: ic_cdk::api::caller(),
    };
    DOCTORS.with(|d| d.borrow_mut().insert(id, doctor));
}

#[update(name = "addPatient")]
#[candid_method(update)]
fn add_patient(id: String, name: String, age: u8, gender: String) {
    let patient = Patient {
        id: id.clone(),
        name,
        age,
        gender,
        medical_history: Vec::new(),
    };
    PATIENTS.with(|p| p.borrow_mut().insert(id, patient));
}

#[update(name = "addMedicalRecord")]
#[candid_method(update)]
fn add_medical_record(patient_id: String, record_id: String, diagnosis: String, prescription: String, date: String) {
    let caller = ic_cdk::api::caller();
    let doctor_id_opt = DOCTORS.with(|d| {
        d.borrow().iter().find_map(|(_, doc)| {
            if doc.principal == caller {
                Some(doc.id.clone())
            } else {
                None
            }
        })
    });

    match doctor_id_opt {
        Some(doctor_id) => {
            PATIENTS.with(|p| {
                let mut patients = p.borrow_mut();
                if let Some(patient) = patients.get_mut(&patient_id) {
                    let record = MedicalRecord {
                        record_id,
                        diagnosis,
                        prescription,
                        date,
                        doctor_id,
                    };
                    patient.medical_history.push(record);
                }
            });
        }
        None => {
            ic_cdk::println!("Unauthorized: Caller is not a registered doctor.");
        }
    }
}

#[query(name = "getPatient")]
#[candid_method(query)]
fn get_patient(id: String) -> Option<Patient> {
    PATIENTS.with(|p| p.borrow().get(&id).cloned())
}

#[query(name = "getAllPatients")]
#[candid_method(query)]
fn get_all_patients() -> Vec<Patient> {
    PATIENTS.with(|p| p.borrow().values().cloned().collect())
}

#[query(name = "getDoctor")]
#[candid_method(query)]
fn get_doctor(id: String) -> Option<Doctor> {
    DOCTORS.with(|d| d.borrow().get(&id).cloned())
}

// Required to expose candid interface
candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid_interface() -> String {
    __export_service()
}
