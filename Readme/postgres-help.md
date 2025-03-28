## **PostgreSQL Commands Documentation**

PostgreSQL is a powerful, open-source relational database system. Below is a concise reference guide for common PostgreSQL commands.

---

### **1. Connection and Session Management**

- **Connect to a PostgreSQL database:**
  ```bash
  psql -U <username> -d <database> -h <host> -p <port>
  ```
  Example:
  ```bash
  psql -U postgres -d mydb -h localhost -p 5432
  ```

- **Switch to a different database within psql:**
  ```sql
  \c <database_name>
  ```
  Example:
  ```sql
  \c mydb
  ```

- **Exit the PostgreSQL session:**
  ```bash
  \q
  ```

---

### **2. Database Management**

- **List all databases:**
  ```sql
  \l
  ```

- **Create a new database:**
  ```sql
  CREATE DATABASE <database_name>;
  ```

- **Drop a database:**
  ```sql
  DROP DATABASE <database_name>;
  ```

- **Rename a database:**
  ```sql
  ALTER DATABASE <old_name> RENAME TO <new_name>;
  ```

---

### **3. User Management**

- **List all users/roles:**
  ```sql
  \du
  ```

- **Create a new user:**
  ```sql
  CREATE USER <username> WITH PASSWORD '<password>';
  ```

- **Grant privileges to a user:**
  ```sql
  GRANT ALL PRIVILEGES ON DATABASE <database_name> TO <username>;
  ```

- **Change a user's password:**
  ```sql
  ALTER USER <username> WITH PASSWORD '<new_password>';
  ```

- **Delete a user:**
  ```sql
  DROP USER <username>;
  ```

---

### **4. Table Management**

- **List all tables in the current database:**
  ```sql
  \dt
  ```

- **Create a new table:**
  ```sql
  CREATE TABLE <table_name> (
      column1 datatype,
      column2 datatype,
      ...
  );
  ```

- **Delete a table:**
  ```sql
  DROP TABLE <table_name>;
  ```

- **View table schema:**
  ```sql
  \d <table_name>
  ```

---

### **5. Querying Data**

- **Insert data into a table:**
  ```sql
  INSERT INTO <table_name> (column1, column2, ...) VALUES (value1, value2, ...);
  ```

- **Retrieve data from a table:**
  ```sql
  SELECT * FROM <table_name>;
  ```

- **Update data in a table:**
  ```sql
  UPDATE <table_name> SET column1 = value1 WHERE condition;
  ```

- **Delete data from a table:**
  ```sql
  DELETE FROM <table_name> WHERE condition;
  ```

---

### **6. Backup and Restore**

- **Backup a database to a file:**
  ```bash
  pg_dump -U <username> -d <database_name> -f <output_file>
  ```
  Example:
  ```bash
  pg_dump -U postgres -d mydb -f mydb_backup.sql
  ```

- **Restore a database from a file:**
  ```bash
  psql -U <username> -d <database_name> -f <input_file>
  ```
  Example:
  ```bash
  psql -U postgres -d mydb -f mydb_backup.sql
  ```

---

### **7. Index Management**

- **Create an index:**
  ```sql
  CREATE INDEX <index_name> ON <table_name> (column_name);
  ```

- **Drop an index:**
  ```sql
  DROP INDEX <index_name>;
  ```

---

### **8. View and Sequence Management**

- **Create a view:**
  ```sql
  CREATE VIEW <view_name> AS SELECT columns FROM <table_name> WHERE condition;
  ```

- **Drop a view:**
  ```sql
  DROP VIEW <view_name>;
  ```

- **Reset a sequence:**
  ```sql
  ALTER SEQUENCE <sequence_name> RESTART WITH <value>;
  ```

---

### **9. Miscellaneous Commands**

- **Show the current database:**
  ```sql
  SELECT current_database();
  ```

- **Show the current user:**
  ```sql
  SELECT current_user;
  ```

- **Show server version:**
  ```sql
  SELECT version();
  ```

- **Enable query timing:**
  ```sql
  \timing
  ```

- **Clear the screen in psql:**
  ```bash
  \! clear
  ```

---

This reference provides essential PostgreSQL commands. For advanced use cases like replication or tuning, consult the official PostgreSQL documentation. Let me know if you'd like to expand on any section!
