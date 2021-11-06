import pandas as pd
import batch_jaro_winkler as bjw
import numpy as np

df_a = pd.read_csv("./input/prepped_df_a.csv", header=None)
df_b = pd.read_csv("./input/prepped_df_b.csv", header=None)

a_l_names = list(df_a[7].replace(np.nan, '', regex=True))
a_f_names = list(df_a[6].replace(np.nan, '', regex=True))
b_l_names = df_b[7].replace(np.nan, '', regex=True)
b_f_names = df_b[6].replace(np.nan, '', regex=True)

l_exportable_model = bjw.build_exportable_model(a_l_names, nb_runtime_threads=10)
l_runtime_model = bjw.build_runtime_model(l_exportable_model)

f_exportable_model = bjw.build_exportable_model(a_f_names, nb_runtime_threads=10)
f_runtime_model = bjw.build_runtime_model(f_exportable_model)

for name in b_f_names[:1000]:
    bjw.jaro_winkler_distance(f_runtime_model, name, min_score=0.8)
