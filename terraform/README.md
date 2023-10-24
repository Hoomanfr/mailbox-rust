**Setup Terraform to Work with GCP**
To use terraform working with GCP you need:
- Create a bucket with unique name universally
- Create Service account for terraform to work with google apis in GCP with role and proper permissions:

    * Service account admin
    * Storage object admin
    * Owner
```
NOTE: This account is different than the service account defined in the service_account.tf file. This account is being used to initialize GCP for very beginning of the project. you need to download the key json of this file and set it in your GCP_CREDENTIAL in the CICD pipeline.
```

Enable the following GCP services:
- Bucket service
- Artifact registry service
- compute engine api
- Kubernetes Engine
- Cloud Resource Manager API
- Service Networking API
- Cloud SQL Admin API