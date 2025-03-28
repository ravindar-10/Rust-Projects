## **Podman Commands Documentation**

Podman is a daemonless container management tool designed to provide functionality similar to Docker. Below is a concise reference guide for common Podman commands.

---

### **1. General Commands**

- **List all containers:**
  ```bash
  podman ps -a
  ```
- **Run a new container:**
  ```bash
  podman run --name <container-name> -d <image>
  ```
  Example:
  ```bash
  podman run --name my-container -d ubuntu:20.04
  ```

- **Remove a container:**
  ```bash
  podman rm <container-name>
  ```

- **Stop a container:**
  ```bash
  podman stop <container-name>
  ```

- **Start a container:**
  ```bash
  podman start <container-name>
  ```

---

### **2. Container State and Information**

- **Inspect a container:**
  ```bash
  podman inspect <container-name>
  ```

- **Get resource usage (CPU, memory, etc.):**
  ```bash
  podman stats <container-name>
  ```

- **List running containers:**
  ```bash
  podman ps
  ```

- **View logs of a container:**
  ```bash
  podman logs <container-name>
  ```

---

### **3. Container Filesystem**

- **Execute a command in a running container:**
  ```bash
  podman exec -it <container-name> <command>
  ```
  Example:
  ```bash
  podman exec -it my-container bash
  ```

- **Copy a file into a container:**
  ```bash
  podman cp <local-file> <container-name>:<container-path>
  ```
  Example:
  ```bash
  podman cp myfile.txt my-container:/root/
  ```

- **Copy a file from a container:**
  ```bash
  podman cp <container-name>:<container-path> <local-path>
  ```
  Example:
  ```bash
  podman cp my-container:/root/myfile.txt .
  ```

---

### **4. Network Management**

- **List available networks:**
  ```bash
  podman network ls
  ```

- **Inspect a network:**
  ```bash
  podman network inspect <network-name>
  ```

---

### **5. Images**

- **List all images:**
  ```bash
  podman images
  ```

- **Pull an image:**
  ```bash
  podman pull <image>
  ```
  Example:
  ```bash
  podman pull ubuntu:20.04
  ```

- **Remove an image:**
  ```bash
  podman rmi <image>
  ```

---

### **6. Pods**

- **Create a new pod:**
  ```bash
  podman pod create --name <pod-name>
  ```

- **List all pods:**
  ```bash
  podman pod ps
  ```

- **Start a pod:**
  ```bash
  podman pod start <pod-name>
  ```

- **Stop a pod:**
  ```bash
  podman pod stop <pod-name>
  ```

- **Remove a pod:**
  ```bash
  podman pod rm <pod-name>
  ```

---

### **7. Volumes**

- **Create a volume:**
  ```bash
  podman volume create <volume-name>
  ```

- **List volumes:**
  ```bash
  podman volume ls
  ```

- **Remove a volume:**
  ```bash
  podman volume rm <volume-name>
  ```

---

### **8. Miscellaneous**

- **Rename a container:**
  ```bash
  podman rename <old-name> <new-name>
  ```

- **Commit changes to an image:**
  ```bash
  podman commit <container-name> <new-image-name>
  ```

- **Save an image to a file:**
  ```bash
  podman save -o <file-name> <image>
  ```

- **Load an image from a file:**
  ```bash
  podman load -i <file-name>
  ```

- **Remove all stopped containers:**
  ```bash
  podman container prune
  ```

---

This reference guide provides a foundation for working with Podman. Let me know if you’d like detailed examples for any specific use case!
