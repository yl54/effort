# This script will run an curl to the OSS server to create a GCP project.

# Setup initial variables of the curl.
# ep_time=10
# dashboard_id=123
# text="text thing"
localhost_url="localhost:8080"

# Headers
username="yl54@somewhere.com";

# Format the body if necessary. One example of a potentially useful body is below.
body="{\"username\":\"${username}\", \"localhost_url\":\"${localhost_url}\"}";

# Send the curl request somewhere. An example is below 
# curl \
#    -H "username: ${username}" \
#    -H "apikey: application/json" \
#    -H "Content-Type: application/json" \
#    -X POST -d "${body}" \
#    $full_url;
curl \
    -H "username: ${username}" \
    -H "Content-Type: application/json" \
    -X POST -d "${body}" \
    $localhost_url;