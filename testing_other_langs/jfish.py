import pandas as pd
import jellyfish
import numpy as np
import time

df_a = pd.read_csv("./tests/input/prepped_df_a.csv", header=None)
df_b = pd.read_csv("./tests/input/prepped_df_b.csv", header=None)

a_f_names = list(df_a[6])
b_f_names = list(df_b[6])
#b_f_names = df_b[6].replace(np.nan, '', regex=True)

#a_f_names = [n for n in a_f_names if len(n) > 0]
#b_f_names = [n for n in b_f_names if len(n) > 0]


start = time.time_ns() // 1_000_000
for i_b,name_b in enumerate(b_f_names[:10]):
    with open(f"tests/answer/{i_b}.txt", 'w') as f:
        for i_a,name_a in enumerate(a_f_names[:100000]):
            if not isinstance(name_a, str): breakpoint()
            jw = jellyfish.jaro_winkler_similarity(name_b, name_a)
            if jw > 0.0:
                f.write(f"{i_a},{jw:0.2f}\n")

end = time.time_ns() // 1_000_000
print(end - start)

#for name in b_f_names[:1000]:
#    bjw.jaro_winkler_distance(f_runtime_model, name, min_score=0.8)
