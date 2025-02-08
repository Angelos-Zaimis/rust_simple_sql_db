import requests
import time

BASE_URL = "http://127.0.0.1:3030"
TEST_DELAY = 1

def find_table(table_name):
    """Finds an existing table via API."""
    url = f"{BASE_URL}/find/{table_name}"
    try:
        response = requests.post(url)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        return {"error": str(e)}

def delete_row(table_name, row_id):
    """Deletes a row from the database via API."""
    url = f"{BASE_URL}/delete/{table_name}/{row_id}"
    try:
        response = requests.delete(url)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        return {"error": str(e)}

def insert_row(table_name, row_data):
    """Inserts a row into the database via API."""
    url = f"{BASE_URL}/insert/{table_name}"
    try:
        response = requests.post(url, json=row_data)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        return {"error": str(e)}

def update_row(table_name, row_id, new_name, new_age):
    """Updates an existing row in the database via API."""
    url = f"{BASE_URL}/update/{table_name}/{row_id}"
    data = {"name": new_name, "age": new_age}
    try:
        response = requests.put(url, json=data)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        return {"error": str(e)}

def test_database():
    print("\n🚀 Testing existing Database...\n")

    table_name = "users"

    print(f"Finding table: {table_name}...")
    print("Finding table response:", find_table(table_name))

    time.sleep(TEST_DELAY)
    print("\n✏️ Updating Age 30:")
    print("📝 Update Response:", update_row(table_name, 1, "Alice Updated", 30))

    # Delete a Row
    time.sleep(TEST_DELAY)
    print("\n🗑️ Deleting Row ID 2:")
    print("🗑️ Delete Response:", delete_row(table_name, 2))

if __name__ == "__main__":
    test_database()
