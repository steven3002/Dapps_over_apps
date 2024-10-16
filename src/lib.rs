#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U8, U32 };
use stylus_sdk::{ prelude::*, block, msg };

sol_storage! {
    #[entrypoint]
    pub struct ClassState {
        mapping(address => bool) admin;
        mapping( uint8 => Classes) class_x;
        uint8 old_index;
    }

    pub struct Classes {
    string name;
    string  meta_data;
    uint32 created_at;
    uint8 materials_index;
    mapping(uint8 => Material) materials;
    }
    
    pub struct Material{
        string meta_data;
    }

}

#[public]
impl ClassState {
    pub fn create_class(&mut self, name: String, meta_data: String) {
        if !self.is_admin(msg::sender()) {
            return;
        }
        let old_index = self.old_index.get();
        let available_index = old_index;

        let mut new_classes = self.class_x.setter(available_index);

        new_classes.name.set_str(name.clone());
        new_classes.meta_data.set_str(meta_data);
        new_classes.created_at.set(U32::from(block::timestamp()));

        let new_index = old_index + U8::from(1);
        self.old_index.set(new_index);
    }

    pub fn get_class(&self, index: u8) -> String {
        let classes = self.class_x.get(U8::from(index));
        return format!(
            r#"{{"name":"{}","meta_data":"{}","created_at":{}}}"#,
            classes.name.get_string(),
            classes.meta_data.get_string(),
            classes.created_at.get()
        );
    }

    pub fn add_material(&mut self, meta_data: String, index: u8) {
        if !self.is_admin(msg::sender()) {
            return;
        }
        let mut classes = self.class_x.setter(U8::from(index));
        let current_index = classes.materials_index.get();
        let mut new_material = classes.materials.setter(current_index);
        new_material.meta_data.set_str(meta_data);
        let new_index = current_index + U8::from(1);
        classes.materials_index.set(new_index);
    }

    pub fn modify_material_by_index(
        &mut self,
        classes_index: u8,
        material_index: u8,
        meta_data: String
    ) {
        if !self.is_admin(msg::sender()) {
            return;
        }
        let mut classes = self.class_x.setter(U8::from(classes_index));
        let mut material = classes.materials.setter(U8::from(material_index));
        material.meta_data.set_str(meta_data);
    }

    pub fn get_material_by_index(&self, classes_index: u8, material_index: u8) -> String {
        let classes = self.class_x.get(U8::from(classes_index));
        let material = classes.materials.get(U8::from(material_index));
        material.meta_data.get_string()
    }

    pub fn get_material_last_index(&self, index: u8) -> u8 {
        let classes = self.class_x.get(U8::from(index));
        let current_index = classes.materials_index.get();
        let result: u8 = current_index.to::<u8>();
        result
    }

    pub fn get_last_index(&self) -> u8 {
        let index = self.old_index.get();
        let result: u8 = index.to::<u8>();
        result
    }

    pub fn set_admin(&mut self, address: Address) {
        let mut set_admin = self.admin.setter(address);
        set_admin.set(true);
    }

    pub fn is_admin(&self, address: Address) -> bool {
        self.admin.get(address)
    }
}
