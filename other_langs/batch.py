# This file is part of the IPUMS's psuedo_jaro_winkler.
# For copyright and licensing information, see the NOTICE and LICENSE files
# in this project's top-level directory, and also on-line at:
#   https://github.com/mnpopcenter/psuedo_jaro_winkler

import batch_jaro_winkler as bjw
import time

with open("../input/file_a_small.txt") as f:
    names_a = [l.strip() for l in f.readlines()]

with open("../input/file_b.txt") as f:
    names_b = [l.strip() for l in f.readlines()]

f_exportable_model = bjw.build_exportable_model(names_b, nb_runtime_threads=1)
f_runtime_model = bjw.build_runtime_model(f_exportable_model)



start = time.time_ns() // 1_000_000
for i,name in enumerate(names_a):
    with open(f"output/{i}.txt", 'w') as f:
        matches = bjw.jaro_winkler_distance(f_runtime_model, name, min_score=0.8)
        for match in matches:
            f.write(match[0]);

end = time.time_ns() // 1_000_000
print(end - start)

