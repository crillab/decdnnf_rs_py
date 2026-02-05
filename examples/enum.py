import decdnnf_rs_py
import sys


def print_model(m):
    print(m)


def main(path, **kwargs):
    r"""
    Enumerates the models of a Decision-DNNF

    :param path: the path to the file containing the Decision-DNNF
    :type path: str
    :param kwargs: see below

    :Keyword Arguments:
        * *n-vars* the effective number of variables (must be greater than or equal to the number of variables declared in the Decision-DNNF)
    """
    ddnnf = decdnnf_rs_py.PyDecisionDNNF(path)
    if "n-vars" in kwargs:
        ddnnf.update_n_vars(int(kwargs["n-vars"]))
    ddnnf.enumerate_models(print_model)


if __name__ == '__main__':
    main(sys.argv[1], **dict(arg.split('=') for arg in sys.argv[2:]))
