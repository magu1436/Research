
with open("result/result.csv", "r") as f:
    result = [float(v.strip()) for v in f.readline().split(",")]
    
    m = {
        "index": None,
        "max": -1
    }
    for i, v in enumerate(result):
        if v > m["max"]:
            m["index"] = i
            m["max"] = v
    
    print(m)