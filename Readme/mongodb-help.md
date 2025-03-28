## **MongoDB Commands Documentation**

MongoDB is a NoSQL database that uses JSON-like documents with optional schemas. Below is a concise reference guide for common MongoDB commands.

---

### **1. Connection and Session Management**

- **Connect to a MongoDB instance:**
  ```bash
  mongo --host <host> --port <port>
  ```
  Example:
  ```bash
  mongo --host localhost --port 27017
  ```

- **Connect to a specific database:**
  ```bash
  mongo <database_name>
  ```
  Example:
  ```bash
  mongo mydb
  ```

- **Exit the MongoDB shell:**
  ```bash
  exit
  ```

---

### **2. Database Management**

- **Show all databases:**
  ```javascript
  show dbs
  ```

- **Create or switch to a database:**
  ```javascript
  use <database_name>
  ```
  Example:
  ```javascript
  use mydb
  ```

- **Drop a database:**
  ```javascript
  db.dropDatabase()
  ```

---

### **3. Collection Management**

- **Show all collections in the current database:**
  ```javascript
  show collections
  ```

- **Create a new collection:**
  ```javascript
  db.createCollection("<collection_name>")
  ```
  Example:
  ```javascript
  db.createCollection("users")
  ```

- **Drop a collection:**
  ```javascript
  db.<collection_name>.drop()
  ```
  Example:
  ```javascript
  db.users.drop()
  ```

---

### **4. Insert Operations**

- **Insert a single document:**
  ```javascript
  db.<collection_name>.insertOne({
      key1: value1,
      key2: value2,
      ...
  })
  ```
  Example:
  ```javascript
  db.users.insertOne({ name: "Alice", age: 25 })
  ```

- **Insert multiple documents:**
  ```javascript
  db.<collection_name>.insertMany([
      { key1: value1, key2: value2, ... },
      { key1: value1, key2: value2, ... }
  ])
  ```
  Example:
  ```javascript
  db.users.insertMany([
      { name: "Bob", age: 30 },
      { name: "Charlie", age: 35 }
  ])
  ```

---

### **5. Querying Data**

- **Find all documents in a collection:**
  ```javascript
  db.<collection_name>.find()
  ```
  Example:
  ```javascript
  db.users.find()
  ```

- **Find documents with a condition:**
  ```javascript
  db.<collection_name>.find({ key: value })
  ```
  Example:
  ```javascript
  db.users.find({ age: { $gt: 30 } })
  ```

- **Find a single document:**
  ```javascript
  db.<collection_name>.findOne({ key: value })
  ```
  Example:
  ```javascript
  db.users.findOne({ name: "Alice" })
  ```

---

### **6. Update Operations**

- **Update a single document:**
  ```javascript
  db.<collection_name>.updateOne(
      { condition_key: condition_value },
      { $set: { key_to_update: new_value } }
  )
  ```
  Example:
  ```javascript
  db.users.updateOne(
      { name: "Alice" },
      { $set: { age: 26 } }
  )
  ```

- **Update multiple documents:**
  ```javascript
  db.<collection_name>.updateMany(
      { condition_key: condition_value },
      { $set: { key_to_update: new_value } }
  )
  ```
  Example:
  ```javascript
  db.users.updateMany(
      { age: { $gt: 30 } },
      { $set: { status: "Senior" } }
  )
  ```

---

### **7. Delete Operations**

- **Delete a single document:**
  ```javascript
  db.<collection_name>.deleteOne({ key: value })
  ```
  Example:
  ```javascript
  db.users.deleteOne({ name: "Alice" })
  ```

- **Delete multiple documents:**
  ```javascript
  db.<collection_name>.deleteMany({ key: value })
  ```
  Example:
  ```javascript
  db.users.deleteMany({ age: { $lt: 25 } })
  ```

---

### **8. Index Management**

- **Create an index:**
  ```javascript
  db.<collection_name>.createIndex({ key: 1 })
  ```
  Example:
  ```javascript
  db.users.createIndex({ name: 1 })
  ```

- **View all indexes:**
  ```javascript
  db.<collection_name>.getIndexes()
  ```

- **Drop an index:**
  ```javascript
  db.<collection_name>.dropIndex("<index_name>")
  ```

---

### **9. Backup and Restore**

- **Backup a database:**
  ```bash
  mongodump --db <database_name> --out <output_directory>
  ```
  Example:
  ```bash
  mongodump --db mydb --out /backup/
  ```

- **Restore a database:**
  ```bash
  mongorestore --db <database_name> <backup_directory>
  ```
  Example:
  ```bash
  mongorestore --db mydb /backup/mydb/
  ```

---

### **10. Miscellaneous Commands**

- **Show the current database:**
  ```javascript
  db
  ```

- **Count documents in a collection:**
  ```javascript
  db.<collection_name>.countDocuments()
  ```
  Example:
  ```javascript
  db.users.countDocuments()
  ```

- **Get database statistics:**
  ```javascript
  db.stats()
  ```

- **Get collection statistics:**
  ```javascript
  db.<collection_name>.stats()
  ```
  Example:
  ```javascript
  db.users.stats()
  ```

---

This reference provides essential MongoDB commands for everyday use. Let me know if you'd like to expand or clarify any section!
