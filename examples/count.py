import decdnnf_rs_py
import sys

#
# Counts the models of a Decision-Dnnf.
#
# usage: count.py instance.dnnf [n_vars]
#


def main():
    ddnnf = decdnnf_rs_py.PyDecisionDNNF(sys.argv[1])
    if len(sys.argv) > 2:
        ddnnf.update_n_vars(int(sys.argv[2]))
    print(ddnnf.count_models())


if __name__ == '__main__':
    main()
