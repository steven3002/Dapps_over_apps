# Dapps_over_apps
 
# Class and Material Management Smart Contract

## Overview
This smart contract is designed for managing classes and their associated materials in a decentralized system. It includes functionalities for creating classes, adding materials, and managing members and admins. Only admins are allowed to create and modify classes and materials, while registered members can view class details and materials.

## Features
- **Class Creation:** Admins can create classes with a name, metadata, and timestamp.
- **Material Management:** Admins can add and modify materials associated with each class.
- **Member Registration:** Users can register as members to gain access to class information.
- **Admin Management:** Admins can be assigned, and only admins can manage classes and materials.

## Contract Structure
### `ClassState` Struct
This struct manages the overall state of the contract, including:
- `admin`: A mapping of addresses that indicates admin privileges.
- `class_x`: A mapping that stores `Classes` structs by index.
- `members`: A mapping of addresses that indicates registered members.
- `old_index`: Tracks the last used index for class creation.

### `Classes` Struct
Each class contains:
- `name`: The name of the class.
- `meta_data`: Additional metadata related to the class.
- `created_at`: The timestamp when the class was created.
- `materials_index`: The index to track the materials added to the class.
- `materials`: A mapping of material metadata indexed by the material number.

### `Material` Struct
Each material contains:
- `meta_data`: Metadata related to the material.

## Events
- **`Member(address indexed user)`:** This event is triggered when a new user registers as a member.

## Functions
### Admin Functions
- **`create_class(name: String, meta_data: String):`** Creates a new class with the given name and metadata, setting the creation time to the current block timestamp. Only callable by admins.
- **`add_material(meta_data: String, index: u8):`** Adds material metadata to a class at the specified index. Only callable by admins.
- **`modify_material_by_index(class_index: u8, material_index: u8, meta_data: String):`** Modifies the metadata of an existing material. Only callable by admins.
- **`set_admin(address: Address):`** Assigns admin privileges to the specified address.

### Member Functions
- **`register_user():`** Registers the calling user as a member and triggers the `Member` event.
- **`get_class(index: u8):`** Returns the class details (name, metadata, creation time) for the specified class index.
- **`get_material_by_index(classes_index: u8, material_index: u8):`** Returns the material metadata for the given class and material index.
- **`get_material_last_index(index: u8):`** Returns the last unused material index for a given class.
- **`get_last_index():`** Returns the last unused class index.

### Utility Functions
- **`is_member():`** Checks if the calling user is a registered member.
- **`is_admin(address: Address):`** Checks if the specified address is an admin.

## Usage
### Deployment
1. Compile and deploy the contract on your preferred EVM-compatible blockchain.
2. Assign admin roles by calling `set_admin(address)` with the desired addresses.

### Class Management
1. As an admin, create a class using `create_class(name, meta_data)`.
2. Add materials to the class with `add_material(meta_data, index)`.

### Member Registration
1. Users can register as members by calling `register_user()`.
2. Once registered, members can view class information with `get_class(index)` and materials with `get_material_by_index(classes_index, material_index)`.

## Requirements
- The contract is designed for use with the **Stylus SDK** and requires the **Arbitrum** platform.
- Basic understanding of Solidity, Rust, and smart contract development is recommended.

## Events
- **Member(address indexed user):** Emitted when a new member registers.
