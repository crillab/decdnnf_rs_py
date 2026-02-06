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
        * *assumptions* search for models containing all the literals (as integers) of this list
        * *compact* a Boolean value indicating whether models must be compacted (literals replaced by `None` if both values leads to a model)
    """
    ddnnf = decdnnf_rs_py.PyDecisionDNNF(path)
    if "n-vars" in kwargs:
        ddnnf.update_n_vars(int(kwargs["n-vars"]))
    new_kwargs = {}
    if "assumptions" in kwargs:
        str_lits = kwargs["assumptions"].split(',')
        new_kwargs["assumptions"] = [int(x) for x in str_lits]
    if "compact" in kwargs:
        if kwargs["compact"] in ["false", "False", "off", "0"]:
            new_kwargs["compact"] = False
        else:
            new_kwargs["compact"] = True
    ddnnf.enumerate_models(print_model, **new_kwargs)


if __name__ == '__main__':
    main(sys.argv[1], **dict(arg.split('=') for arg in sys.argv[2:]))
