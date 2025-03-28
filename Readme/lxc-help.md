## **LXC Commands Documentation**

LXC (Linux Containers) provides a lightweight virtualization solution to create and manage containers. Below is a concise reference guide for common LXC commands.

---

### **1. General Commands**

- **List all containers:**
  ```bash
  lxc list
  ```
- **Launch a new container:**
  ```bash
  lxc launch <image> <container-name>
  ```
  Example:
  ```bash
  lxc launch ubuntu:20.04 my-container
  ```

- **Delete a container:**
  ```bash
  lxc delete <container-name>
  ```

- **Stop a container:**
  ```bash
  lxc stop <container-name>
  ```

- **Start a container:**
  ```bash
  lxc start <container-name>
  ```

---

### **2. Container State and Information**

- **Get detailed state information for a container:**
  ```bash
  lxc query /1.0/containers/<container-name>/state
  ```

- **View resource usage (CPU, memory, etc.):**
  ```bash
  lxc info <container-name>
  ```

---

### **3. Container Filesystem**

- **Execute a command in a running container:**
  ```bash
  lxc exec <container-name> -- <command>
  ```
  Example:
  ```bash
  lxc exec my-container -- bash
  ```

- **Push a file into a container:**
  ```bash
  lxc file push <local-file> <container-name>/<container-path>
  ```
  Example:
  ```bash
  lxc file push myfile.txt my-container/root/
  ```

- **Pull a file from a container:**
  ```bash
  lxc file pull <container-name>/<container-path> <local-path>
  ```
  Example:
  ```bash
  lxc file pull my-container/root/myfile.txt .
  ```

---

### **4. Network Management**

- **List available networks:**
  ```bash
  lxc network list
  ```

- **Get detailed information about a network:**
  ```bash
  lxc network show <network-name>
  ```

---

### **5. Snapshots and Backups**

- **Create a snapshot:**
  ```bash
  lxc snapshot <container-name> <snapshot-name>
  ```

- **List snapshots:**
  ```bash
  lxc info <container-name>
  ```

- **Restore a snapshot:**
  ```bash
  lxc restore <container-name> <snapshot-name>
  ```

---

### **6. Managing Container Profiles**

- **List profiles:**
  ```bash
  lxc profile list
  ```

- **Apply a profile to a container:**
  ```bash
  lxc profile assign <container-name> <profile-name>
  ```

---

### **7. Importing and Exporting Containers**

- **Export a container:**
  ```bash
  lxc export <container-name> <file-name>
  ```

- **Import a container:**
  ```bash
  lxc import <file-name>
  ```

---

### **8. Miscellaneous**

- **Create a container from an image:**
  ```bash
  lxc init <image> <container-name>
  ```

- **Rename a container:**
  ```bash
  lxc rename <old-container-name> <new-container-name>
  ```

- **Update container configuration:**
  ```bash
  lxc config set <container-name> <key> <value>
  ```
  Example:
  ```bash
  lxc config set my-container limits.cpu 2
  ```

---

This quick reference should help with common tasks in managing LXC containers. Let me know if you’d like detailed examples for any command!
