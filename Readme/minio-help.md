**MinIO Commands Cheatsheet**  

1. **Create Bucket**  
   `mc mb ALIAS/BUCKET_NAME`  

2. **Delete Bucket**  
   `mc rb --force ALIAS/BUCKET_NAME`  

3. **Bucket Encryption**  
   `mc encrypt set ALIAS/BUCKET_NAME`  

4. **Enable/Disable/Check Encryption**  
   - Enable: `mc encrypt set ALIAS/BUCKET_NAME`  
   - Disable: `mc encrypt clear ALIAS/BUCKET_NAME`  
   - Info: `mc encrypt info ALIAS/BUCKET_NAME`  

5. **List Buckets**  
   `mc ls ALIAS`  

6. **Object Operations**  
   - Upload: `mc cp FILE ALIAS/BUCKET_NAME/PATH`  
   - Download: `mc cp ALIAS/BUCKET_NAME/OBJECT .`  
   - List: `mc ls ALIAS/BUCKET_NAME/PATH`  
   - Delete: `mc rm ALIAS/BUCKET_NAME/OBJECT`  

7. **Bucket Replication**  
   - Enable/Add: `mc replicate add ALIAS/BUCKET_NAME TARGET_ALIAS/TARGET_BUCKET`  
   - Update: `mc replicate update ALIAS/BUCKET_NAME`  
   - Remove: `mc replicate remove ALIAS/BUCKET_NAME`  
   - List: `mc replicate ls ALIAS/BUCKET_NAME`  

8. **Bucket Retention**  
   - Enable: `mc retention set --mode GOVERNANCE/COMPLIANCE ALIAS/BUCKET_NAME`  
   - Disable: `mc retention clear ALIAS/BUCKET_NAME`  
   - Info: `mc retention info ALIAS/BUCKET_NAME`  

9. **Generate Shareable Object Link**  
   `mc share download ALIAS/BUCKET_NAME/OBJECT`  

10. **Versioning**  
    - Enable: `mc version enable ALIAS/BUCKET_NAME`  
    - Disable: `mc version suspend ALIAS/BUCKET_NAME`  
    - Check Status: `mc version info ALIAS/BUCKET_NAME`  

**Notes**:  
- Replace `ALIAS` with your MinIO alias (e.g., `myminio`)  
- Use `--recursive` flag for bulk operations (e.g., `mc rm --recursive`).  
- For API access, use the `mc` CLI with `admin` credentials.
