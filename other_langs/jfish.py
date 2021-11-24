# This file is part of the IPUMS's psuedo_jaro_winkler.
# For copyright and licensing information, see the NOTICE and LICENSE files
# in this project's top-level directory, and also on-line at:
#   https://github.com/mnpopcenter/psuedo_jaro_winkler

import pandas as pd
import jellyfish
import numpy as np
import time

#df_a = pd.read_csv("./tests/input/prepped_df_a.csv", header=None)
#df_b = pd.read_csv("./tests/input/prepped_df_b.csv", header=None)
#
#a_f_names = list(df_a[6])
#b_f_names = list(df_b[6])
#b_f_names = df_b[6].replace(np.nan, '', regex=True)

#a_f_names = [n for n in a_f_names if len(n) > 0]
#b_f_names = [n for n in b_f_names if len(n) > 0]

with open("../input/file_a_small.txt") as f:
    names_a = f.readlines()

with open("../input/file_b.txt") as f:
    names_b = f.readlines()


start = time.time_ns() // 1_000_000
for i_a,name_a in enumerate(names_a):
    with open(f"output/{i_a}.txt", 'w') as f:
        for i_b,name_b in enumerate(names_b):
            jw = jellyfish.jaro_winkler_similarity(name_a, name_b)
            if jw > 0.8:
                f.write(f"{i_b},{jw:0.2f}\n")

end = time.time_ns() // 1_000_000
print(end - start)

#for name in b_f_names[:1000]:
#    bjw.jaro_winkler_distance(f_runtime_model, name, min_score=0.8)
