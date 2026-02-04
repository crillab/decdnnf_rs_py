import decdnnf_rs_py
import sys

#
# Enumerates the models of a Decision-Dnnf.
#
# usage: enum.py instance.dnnf [n_vars]
#


def print_model(m):
    print(m)


def main():
    ddnnf = decdnnf_rs_py.PyDecisionDNNF(sys.argv[1])
    if len(sys.argv) > 2:
        ddnnf.update_n_vars(int(sys.argv[2]))
    ddnnf.enumerate_models(print_model)


if __name__ == '__main__':
    main()
