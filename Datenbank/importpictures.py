import sqlite3

def convertToBinaryData(filename):
    # Convert digital data to binary format
    with open(filename, 'rb') as file:
        blobData = file.read()
    return blobData

def insertBLOB(prd_name, prd_picture):
    try:
        sqliteConnection = sqlite3.connect('eatyoucate/Datenbank/datenbank.db')
        cursor = sqliteConnection.cursor()
        print("Connected to SQLite")
        sqlite_insert_blob_query = """ update Produkte set PRD_PICTURE = ? where PRD_NAME = ? """

        empPhoto = convertToBinaryData(prd_picture)
        # Convert data into tuple format
        data_tuple = (prd_picture, prd_name)
        cursor.execute(sqlite_insert_blob_query, data_tuple)
        sqliteConnection.commit()
        print("Image and file inserted successfully as a BLOB into a table")
        cursor.close()

    except sqlite3.Error as error:
        print("Failed to insert blob data into sqlite table", error)
    finally:
        if sqliteConnection:
            sqliteConnection.close()
            print("the sqlite connection is closed")

insertBLOB("Apfel", "eatyoucate/Datenbank/Bilder/apple.jpg")
insertBLOB("Banane", "eatyoucate/Datenbank/Bilder/banana.jpg")
insertBLOB("Pizza", "eatyoucate/Datenbank/Bilder/pizza.jpg")
insertBLOB("Brezel", "eatyoucate/Datenbank/Bilder/prezel.jpg")
insertBLOB("Reis", "eatyoucate/Datenbank/Bilder/rice.jpg")
