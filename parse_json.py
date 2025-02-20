#!/usr/bin/env python
import hashlib
import json


def main():
    filename = "ds.json"

    with open(filename) as fobj:
        data = json.load(fobj)

    details = {}
    result = {}
    # for sha1 based look ups
    sha1 = {}
    for i, school in enumerate(data):
        # First get the title for the school
        title = school.get("title", "")
        system_type = school.get("type", "")
        # We need calculate hash
        m = hashlib.sha1()
        m.update(school["entity_id"].encode("utf-8"))
        sha_text = "{sha1}" + m.hexdigest()
        school["id"] = sha_text
        # We added the hash
        details[i] = school
        sha1[f"{sha_text}.json"] = school
        # Let us remove all SPs
        if system_type != "idp":
            continue

        if not title:
            continue
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
    webdata = {"schools": details, "answers": result, "sha1": sha1}
    with open(outputfile, "w") as fobj:
        json.dump(webdata, fobj)
    # 8MB of output
    print(f"Data converted to {outputfile}")


if __name__ == "__main__":
    main()
