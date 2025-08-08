#!/usr/bin/env python
import hashlib
import json
import os
import sys

import httpx


def main():
    "Starts here"
    if len(sys.argv) > 1:
        url = sys.argv[1]
    else:
        url = str(os.getenv("COLLECTION_ENDPOINT"))

    if not url:
        print(
            "No collection endpoint given as argument or via COLLECTION_ENDPOINT environment variable."
        )
    try:
        print("Fetching data from collection endpoint")
        resp = httpx.get(url)
    except Exception as e:
        print(f"Failed to fetch data: {e}")
        sys.exit(1)
    try:
        data = resp.json()
    except Exception as e:
        print(f"Failed to parse data: {e}")
        sys.exit(1)
    entities = data.get("federation_entities", [])

    details = {}
    result = {}
    sha1 = {}
    for i, entity in enumerate(entities):
        print(f"Processing {i}")
        title = str(
            entity.get("ui_infos", {}).get("openid_provider", {}).get("display_name")
        )
        if not title:
            continue
        etypes = entity.get("entity_types", [])
        if "openid_provider" not in etypes:
            continue
        m = hashlib.sha1()
        m.update(entity.get("entity_id").encode("utf-8"))
        sha_text = "{sha1}" + m.hexdigest()
        # Adds sha1 as id in the entry
        entity["id"] = sha_text

        # We added the hash
        details[i] = entity
        # For both with json ending and without
        sha1[f"{sha_text}.json"] = entity
        sha1[f"{sha_text}"] = entity
        title = title.lower()
        length = len(title)
        for start in range(0, length - 1):
            end = start + 1
            while end <= length:  # Has to be smaller for 1 char
                # This is  what the student has typed
                key = title[start:end]
                output = result.get(key, [])
                output = set(output)  # We want uniques
                output.add(i)
                result[key] = list(output)
                end += 1

    # Now we are done
    # save the data
    outputfile = "webdata.json"
    webdata = {"providers": details, "answers": result, "sha1": sha1}
    with open(outputfile, "w") as fobj:
        json.dump(webdata, fobj)
    # 103MB of output
    print(f"Data converted to {outputfile}")


if __name__ == "__main__":
    main()
