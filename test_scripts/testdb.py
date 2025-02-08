import requests
import time

BASE_URL = "http://127.0.0.1:3030"  # Rust API URL
TEST_DELAY = 1  # Delay in seconds between API calls (set to 0 for fast testing)


def create_table(table_name):
    """Creates a table in the database via API."""
    url = f"{BASE_URL}/create"
    data = {"table_name": table_name}  # Send JSON body
    try:
        response = requests.post(url, json=data)
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


def select_all(table_name):
    """Retrieves all rows from a table via API."""
    url = f"{BASE_URL}/select/{table_name}"
    try:
        response = requests.get(url)
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


def delete_row(table_name, row_id):
    """Deletes a row from the database via API."""
    url = f"{BASE_URL}/delete/{table_name}/{row_id}"
    try:
        response = requests.delete(url)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        return {"error": str(e)}


def test_database():
    print("\n🚀 Starting Full API Test...\n")

    table_name = "users"

    # Create Table
    print(f"🛠️ Creating table: {table_name} ...")
    print("🛠️ Create Response:", create_table(table_name))

    # Insert Rows
    print("\n📝 Inserting Rows...")
    row1 = {"id": 1, "name": "Alice", "age": 25}
    row2 = {"id": 2, "name": "Bob", "age": 30}

    print("➕ Insert Response 1:", insert_row(table_name, row1))
    print("➕ Insert Response 2:", insert_row(table_name, row2))

    # Select All Rows
    time.sleep(TEST_DELAY)
    print("\n🔍 Retrieving All Users:")
    print("📋 Select Response:", select_all(table_name))

    # Update a Row
    time.sleep(TEST_DELAY)
    print("\n✏️ Updating Row ID 1:")
    print("📝 Update Response:", update_row(table_name, 1, "Alice Updated", 26))

    # Select Again After Update
    time.sleep(TEST_DELAY)
    print("\n🔍 Retrieving Users After Update:")
    print("📋 Select Response:", select_all(table_name))

    # Delete a Row
    time.sleep(TEST_DELAY)
    print("\n🗑️ Deleting Row ID 2:")
    print("🗑️ Delete Response:", delete_row(table_name, 2))

    # Select Again After Deletion
    time.sleep(TEST_DELAY)
    print("\n🔍 Retrieving Users After Deletion:")
    print("📋 Select Response:", select_all(table_name))

    print("\n📝 Inserting Rows...")
    row1 = {"id": 1, "name": "bil", "age": 25}
    row2 = {"id": 2, "name": "george", "age": 30}

    print("➕ Insert Response 1:", insert_row(table_name, row1))
    print("➕ Insert Response 2:", insert_row(table_name, row2))


    print("\n✅ API Test Completed Successfully!\n")


if __name__ == "__main__":
    test_database()
