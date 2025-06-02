from fastapi import FastAPI, Request
import time
import uuid

app = FastAPI()

current_active_users = {} # all users currently active, which means that a user retrieved a request within the last hour

session_log = {}

@app.get('/')
async def root(request: Request):
    key = uuid.uuid4()
    timestamp = time.time()

    process_session(key, timestamp)
    update_current_users(write_to_log(timestamp, key)) #TODO: Absolute waste of resource to call on each request. Remove clean_log from here and move to cron_job on server

    return session_store


def process_session(key, timestamp):
    session_store[key] = timestamp # insert AND update
    # In regular intervals DELETE everything older than one hour
    return True

def update_current_users(timestamp, key) -> []:
    # updates the log and return list of inactive users
    session_log[timestamp] = key
    one_hour_ago = time.time() - 3600
    # how to retrieve a range of keys from a dict?
    inactive_users = [v for k,v in session_log if k < one_hour_ago and k > one_hour_ago - 3600]
    return inactive_users # List of keys of inactive users
