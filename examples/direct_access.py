import decdnnf_rs_py
import sys


def print_model(m):
    print(m)


def main(path, index, **kwargs):
    r"""
    Enumerates the models of a Decision-DNNF

    :param path: the path to the file containing the Decision-DNNF
    :type path: str
    :param kwargs: see below

    :Keyword Arguments:
        * *n-vars* the effective number of variables (must be greater than or equal to the number of variables declared in the Decision-DNNF)
        * *lexico* a Boolean value indicating whether models must be sorted in the lexicographic order (default is no)
    """
    ddnnf = decdnnf_rs_py.PyDecisionDNNF(path)
    if "n-vars" in kwargs:
        ddnnf.update_n_vars(int(kwargs["n-vars"]))
    new_kwargs = {}
    if "lexico" in kwargs:
        if kwargs["lexico"] in ["false", "False", "off", "0"]:
            new_kwargs["lexico"] = False
        else:
            new_kwargs["lexico"] = True
    ddnnf.direct_access(int(index), print_model, **new_kwargs)


if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2], **dict(arg.split('=')
         for arg in sys.argv[3:]))
