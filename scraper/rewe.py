import sys
from bs4 import BeautifulSoup
import json
import re

with open(sys.argv[1], "r") as f:
    soup = BeautifulSoup(f, "html.parser")
    
entries = []

def parseprice(price):
    return float(price.replace("€","").replace(",",".")) 

def parsegrammage(grammage: str, price=0):
    if "1kg" == grammage:
        return round(price/100)
    if "1 kg" in grammage:
        return round(float(re.search("\(1 kg = (\d+,\d+) €\)", grammage).group(1).replace(",", "."))/10,2)
    return price

for product in soup.find_all("div", "search-service-product"):
    entry = {}

    title = product.find("h4")
    price = product.find("div", "search-service-productPrice")
    if not price:
        price = product.find("div", "search-service-productOfferPrice")
    organic = product.find("div", "search-service-organicBadge")
    grammage = product.find("div", "search-service-productGrammage")

    entry["title"] = title.text
    entry["price"] = parseprice(price.text)
    entry["organic"] = False
    if organic:
        entry["organic"] = True
    entry["grammage"] = parsegrammage(grammage.text, entry["price"])

    entries.append(entry)

print(json.dumps(entries, ensure_ascii=False))
    

        
    