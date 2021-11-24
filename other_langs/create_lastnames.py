# This file is part of the IPUMS's psuedo_jaro_winkler.
# For copyright and licensing information, see the NOTICE and LICENSE files
# in this project's top-level directory, and also on-line at:
#   https://github.com/mnpopcenter/psuedo_jaro_winkler

import pandas as pd
import sys


df = pd.read_csv(sys.argv[1], header=None)

lnames = df[7]
with open(sys.argv[2], 'w') as f:
    for name in lnames:
        if isinstance(name, str) and len(name) != 0:
            f.write(name + "\n");
