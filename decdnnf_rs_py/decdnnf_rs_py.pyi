from typing import Callable, List, Optional


class PyDecisionDNNF:
    """
    A class handling a Decision-DNNF.

    :param path: the path to the file containing the Decision-DNNF
    """

    def __init__(self, path: str) -> None: ...

    def update_n_vars(n_vars: int) -> None:
        """
        Updates the number of variables.

        The new number must be greater than the current number of variables.
        This function is useful when loading a Decision-DNNF in which the last variables are free. In this case, the formula itself is insufficient to deduce the actual number of variables.
        For example, the formula for the trivial, true Decision-DNNF reduces to the constant true, regardless of the number of variables.
        Calling this function indicates the real number of variables that the formula relies on.

        :param n_vars: the new number of variables
        :type n_vars: int
        """

    def model(**kwargs: str) -> Optional[List[int]]:
        """
        Computes a model.

        :return: the model, as a list of literals
        :rtype: (List[int] | None)
        :param kwargs: see below

        :Keyword Arguments:
            * *assumptions* search for models containing all the literals (as integers) of this list
        """

    def count_models(**kwargs: str) -> int:
        """
        Count the models of the Decision-DNNF.

        :return: the number of models of the Decision-DNNF
        :rtype: int
        :param kwargs: see below

        :Keyword Arguments:
            * *assumptions* search for models containing all the literals (as integers) of this list
        """

    def enumerate_models(callback: Callable[[List[Optional[int]]], None], **kwargs: str) -> None:
        r"""
        Enumerates the models of the Decision-DNNF.

        The provided callback function is called with each model.
        A model is given as a list of literals, sorted from the first variable to the last.
        In case `None` replaces a literal, then setting any polarity to the literal leads to a model.

        :param callback: the function to call for each model
        :type callback: (List[bool]) -> None
        :param kwargs: see below

        :Keyword Arguments:
            * *assumptions* search for models containing all the literals (as integers) of this list
            * *compact* a Boolean value indicating whether models must be compacted (literals replaced by `None` if both values leads to a model)
        """

    def direct_access(index: int, callback: Callable[[List[Optional[int]]], None], **kwargs: str) -> None:
        r"""
        Computes the n-th model of the Decision-DNNF.

        The provided callback function is called with the model.
        A model is given as a list of literals, sorted from the first variable to the last.

        The default order is dependant of the structure of the Decision-DNNF in memory.
        In order to sort the models using the lexicographic order, sets the `lexico` flag to True (default order is clearly faster).

        :param index: the index of the model; must be lower than the model count
        :type index: int
        :param callback: the function to call on the model
        :type callback: (List[bool]) -> None
        :param kwargs: see below

        :Keyword Arguments:
            * *lexico* sets the lexicographic order 
        """

    def sampling(n_samples: int, callback: Callable[[List[Optional[int]]], None], **kwargs: str) -> None:
        r"""
        Samples uniformly some models of the Decision-DNNF.

        The provided callback function is called with the models.
        A model is given as a list of literals, sorted from the first variable to the last.

        The default order is dependant of the structure of the Decision-DNNF in memory.
        In order to sort the models using the lexicographic order, sets the `lexico` flag to True (default order is clearly faster).

        :param n_samples: the number of models to sample
        :type n_samples: int
        :param callback: the function to call on the model
        :type callback: (List[bool]) -> None
        :param kwargs: see below

        :Keyword Arguments:
            * *lexico* sets the lexicographic order 
        """
