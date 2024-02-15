
# TodoList API
The TodoList API is a RESTful web service that provides endpoints for managing tasks in a to-do list application. It allows clients to perform CRUD operations (Create, Read, Update, Delete) on tasks, enabling seamless integration with frontend applications or other services.
# Features
1. Create new tasks with a title,date.
2. Retrieve a list of all tasks or specific tasks by ID.
3. Update existing tasks to modify their title.
4. Delete tasks that are no longer needed based on ID.
# Technologies Used
1. Language: Rust
2. Web Framework: Actix Web
# Installation
1. Clone the repository: git clone https://github.com/your/repository.git
2. Navigate to the project directory: cd todolist_api
3. Install dependencies: cargo add actix-web ,cargo add serde --features derive , cargo add serde_json
# Usage
1. Start the server: cargo run.
2. Access the API endpoints using a tool like cURL, Postman, or integrate it with your frontend application.
# API Endpoints
1. GET /todolist/entries: Retrieve all tasks.
2. GET /todolist/entries/{id}: Retrieve a specific task by ID.
3. POST /todolist/entries: Create a new task.
4. PUT /todolist/entries/{id}: Update an existing task by ID.
5. DELETE /todolist/entries/{id}: Delete a task by ID.
# Request and Response Formats
1. Request: JSON format for POST and PUT requests.
2. Response: JSON format containing entries details.
# Error Handling
1. Warning will be returned if the entry is not present for a specific id while updating the entries.
# Contributing
Contributions are welcome! Please fork the repository, make your changes, and submit a pull request.
