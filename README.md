# Culinary Adventure Tracker

## Overview

The Culinary Adventure Tracker is an application for food enthusiasts, allowing users to track their culinary experiences. Users can log details about restaurants visited, rate dishes, and maintain a personalized food map. The app also provides personalized restaurant recommendations based on users' tastes.

## Prerequisites

Before you begin, ensure you have the following installed:

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- Internet Computer SDK: [IC SDK Installation Guide](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html)

## Getting Started

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/culinary-adventure-tracker.git
    cd culinary-adventure-tracker
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Deploy the canister:

    ```bash
    npm run gen-deploy
    ```

## Project Structure

The project is structured as follows:

- `src/lib.rs`: Additional Rust modules and helper functions.

## Functions Hierarchy

1. **`add_culinary_adventure`**
    - Adds a new culinary adventure to the tracker.

2. **`get_culinary_adventure`**
    - Retrieves details of a specific culinary adventure.

3. **`update_culinary_adventure`**
    - Updates details of an existing culinary adventure.

4. **`delete_culinary_adventure`**
    - Deletes a culinary adventure from the tracker.

5. **`get_culinary_adventures_before_date`**
    - Retrieves a list of culinary adventures before a specified date.

6. **`update_visit_date`**
    - Updates the visit date of a culinary adventure.

7. **`get_all_culinary_adventures`**
    - Retrieves details of all culinary adventures in the tracker.

8. **`get_total_culinary_adventures`**
    - Retrieves the total number of culinary adventures.

9. **`get_culinary_adventures_count_before_date`**
    - Retrieves the count of culinary adventures before a specified date.

