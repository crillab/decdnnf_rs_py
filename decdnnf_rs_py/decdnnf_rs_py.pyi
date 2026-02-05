from typing import Callable, List


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

    def count_models() -> int:
        """
        Count the models of the Decision-DNNF.

        :return: the number of models of the Decision-DNNF
        :rtype: int
        """

    def enumerate_models(callback: Callable[[List[bool]], None]) -> None:
        """
        Enumerates the models of the Decision-DNNF.

        The provided callback function is called with each model.

        :param callback: the function to call for each model
        :type callback: (List[bool]) -> None
        """
