import os
import sys
import requests
import argparse
import json
from importlib.util import find_spec
import test

ENTRANCE_IP: str = "wifi.cnu.edu.cn"

LOGIN_NODE: str = "/drcom/login"
LOGIN_CALLBACK: str = "dr1004"
LOGIN_0MKKEY: str = "123456"

LOGOUT_NODE: str = "/drcom/logout"
LOGOUT_CALLBACK: str = "dr1004"

QUERY_USER_INFO_NODE: str = ":802/eportal/portal/custom/loadUserInfo"
QUERY_USER_INFO_CALLBACK: str = "dr1002"
QUERY_ONLINE_DEVICE_NODE: str = ":802/eportal/portal/custom/loadOnlineDevice"
QUERY_ONLINE_DEVICE_CALLBACK: str = "dr1003"


class LoginResult:

    """
    dr1004({
        "result": 1,
        "aolno": 17277,
        "m46": 0,
        "v46ip": "10.1.174.62",
        "myv6ip": "",
        "sms": 0,
        "NID": "",
        "olmac": "7e62ac81517b",
        "ollm": 0,
        "olm1": "00000000",
        "olm2": "0010",
        "olm3": 0,
        "olmm": 2,
        "olm5": 0,
        "gid": 1,
        "ispid": 0,
        "opip": "0.0.0.0",
        "oltime": 172800000,
        "olflow": 4294967295,
        "lip": "",
        "stime": "",
        "etime": "",
        "uid": "1211001011",
        "sv": 0
    })
    """

    def __init__(self, result: int, v46ip: str, olmac: str, uid: str) -> None:
        self.result = result
        self.v46ip = v46ip
        self.olmac = olmac
        self.uid = uid


def make_login_result(d: dict) -> LoginResult:
    return LoginResult(d["result"], d["v46ip"], d["olmac"], d["uid"])


class LogoutResult:
    """
        dr1004({
        "result": 1,
        "wopt": 0,
        "msg": 14,
        "hidm": 0,
        "hidn": -5,
        "ss5": "10.1.174.62",
        "ss6": "192.168.1.91",
        "vid": 0,
        "ss1": "00900b91dee4",
        "ss4": "7e62ac81517b",
        "cvid": 0,
        "pvid": 0,
        "hotel": 0,
        "aolno": 17271,
        "eport": -1,
        "eclass": 1,
        "time": 5,
        "flow": 34848,
        "fsele": 1,
        "fee": 0,
        "v6af": 0,
        "v6df": 0,
        "emark": 0,
        "actM": 1,
        "actt": 30535802,
        "actdf": 32061,
        "actuf": 2787,
        "act6df": 0,
        "act6uf": 0,
        "allfm": 2,
        "d1": 0,
        "u1": 0,
        "d2": 0,
        "u2": 0,
        "o1": 0,
        "nd1": 128244,
        "nu1": 11148,
        "nd2": 0,
        "nu2": 0,
        "no1": 0,
        "uid": ",`,1211001011"
    })
    """

    def __init__(
        self,
        result: int,
        ss5: str,
        ss6: str,
        ss4: str,
        time: int,
        flow: float,
        uid: str,
    ) -> None:
        self.result = result
        self.ss5 = ss5
        self.ss6 = ss6
        self.ss4 = ss4
        self.time = time
        self.flow = flow
        self.uid = uid


def make_logout_result(d: dict) -> LogoutResult:
    return LogoutResult(
        d["result"], d["ss5"], d["ss6"], d["ss4"], d["time"], d["flow"], d["uid"]
    )


class UserInfo:
    def __init__(
        self, user_flow: float, user_time: float, user_money: float, mac: str
    ) -> None:
        self.user_flow = user_flow
        self.user_time = user_time
        self.user_money = user_money
        self.mac = mac


def make_user_info(d: dict) -> UserInfo:
    return UserInfo(d["USERFLOW"], d["USERTIME"], d["USERMONEY"], d["MAC"])


class QueryUserInfoResult:
    """
        dr1005({
        "code": "1",
        "overLimit": "0",
        "data": [{
            "USERFLOW": 81693.842,
            "USERTIME": 19722,
            "USERMONEY": 8.0617,
            "MAC": "04106BFDC230",
            "LIMITCOUNT": 1
        }],
        "msg": "获取Mac绑定数量接口成功"
    });"""

    def __init__(self, code: str, data: UserInfo, msg: str) -> None:
        self.code = code
        self.data = data
        self.msg = msg


def make_query_user_info_result(d: dict) -> QueryUserInfoResult:
    return QueryUserInfoResult(d["code"], make_user_info(d["data"][0]), d["msg"])


class DeviceInfo:
    def __init__(
        self, login_time: str, bas_id: int, login_ip: str, mac_address: str
    ) -> None:
        self.login_time = login_time
        self.bas_id = bas_id
        self.login_ip = login_ip
        self.mac_address = mac_address


def make_device_info(d: dict) -> DeviceInfo:
    return DeviceInfo(d["login_time"], d["bas_id"], d["login_ip"], d["mac_address"])


class QueryDeviceInfoResult:
    """
        dr1006({
        "code": "1",
        "data": [{
            "login_time": "2023-10-26 11:00:00",
            "bas_id": 1,
            "login_ip": "10.1.174.62",
            "mac_address": "7E62AC81517B",
            "session_id": 50079
        }],
        "msg": "获取登录设备信息成功"
    });"""

    def __init__(self, code: str, data: list[DeviceInfo], msg: str) -> None:
        self.code = code
        self.data = data
        self.msg = msg


def make_query_device_info_result(d: dict) -> QueryDeviceInfoResult:
    return QueryDeviceInfoResult(
        d["code"], list(map(make_device_info, d["data"])), d["msg"]
    )


def get_login_url(account: str, password: str) -> str:
    return f"https://{ENTRANCE_IP}{LOGIN_NODE}?callback={LOGIN_CALLBACK}&DDDDD={account}&upass={password}&0MKKey={LOGIN_0MKKEY}"


def get_logout_url() -> str:
    return f"https://{ENTRANCE_IP}{LOGOUT_NODE}?callback={LOGOUT_CALLBACK}"


def get_query_user_info_url(account: str) -> str:
    return f"https://wifi.cnu.edu.cn{QUERY_USER_INFO_NODE}?callback={QUERY_USER_INFO_CALLBACK}&account={account}"


def get_query_online_device_url(account: str) -> str:
    return f"https://wifi.cnu.edu.cn{QUERY_ONLINE_DEVICE_NODE}?callback={QUERY_ONLINE_DEVICE_CALLBACK}&account={account}"


def check_win() -> bool:
    return sys.platform == "win32"


def check_cnu() -> bool:
    if check_win():
        result = os.popen("netsh WLAN show interfaces").readlines()
        return len(result) > 10 and "CNU" in result[9]
    else:
        result = os.popen("nmcli connection show --active").readlines()
        return len(result) > 1 and "CNU" in result[1]


def notify(notification: str) -> None:
    if check_win() and find_spec("win10toast"):
        from win10toast import ToastNotifier

        ToastNotifier().show_toast("CNU NETWORK TOY 校园网工具", notification, duration=10)
    else:
        print(notification)


def query_user_info(account: str) -> QueryUserInfoResult:
    return make_query_user_info_result(
        json.loads(
            requests.get(get_query_user_info_url(account))
            .content.decode("utf-8")
            .strip()[7:-2]
        )
    )


def query_device_info(account: str) -> QueryDeviceInfoResult:
    return make_query_device_info_result(
        json.loads(
            requests.get(get_query_online_device_url(account))
            .content.decode("utf-8")
            .strip()[7:-2]
        )
    )


def handle_login(args):
    response = requests.get(get_login_url(args.account, args.password))
    if response.status_code != 200:
        return notify(f"登录失败。\n错误码: {response.status_code}\n错误信息: {response.content}")

    login_result = make_login_result(
        json.loads(response.content.decode("utf-8").strip()[7:-1])
    )
    if login_result.result != 1:
        return notify(f"登录失败。\n错误码: {login_result.result}")

    user_ip, money, device_count = login_result.v46ip, 0.0, 0

    query_user_info_result = query_user_info(args.account)
    if query_user_info_result.code == "1":
        money = query_user_info_result.data.user_money

    query_device_info_result = query_device_info(args.account)
    if query_device_info_result.code == "1":
        device_count = len(query_device_info_result.data)

    notify(f"{args.account} 登录成功。\n本机IP: {user_ip}\n余额: {money}\n设备数量: {device_count}")


def handle_status(args):
    account = args.account
    query_user_info_result = query_user_info(account)
    print(f"{account}的校园网信息:")
    if query_user_info_result.code == "1":
        user_info = query_user_info_result.data
        print(f"{'已用流量':<21}{'已用时长':<11}{'用户余额':<11}{'无感知MAC':<9}")
        print(
            f"{str(user_info.user_flow)+'MB':<25}{str(user_info.user_time)+'Min':<15}{str(user_info.user_money)+'元':<14}{user_info.mac:<12}"
        )
    else:
        print("用户信息获取失败。", "")

    query_device_info_result = query_device_info(account)
    if query_device_info_result.code == "1":
        print(f"{'登录时间':<21}{'认证服务器':<10}{'设备IP':<13}{'设备MAC':<10}")
        for device in query_device_info_result.data:
            print(
                f"{device.login_time:<25}{device.bas_id:<15}{device.login_ip:<15}{device.mac_address:<12}"
            )
    else:
        print("设备信息获取失败。")


def handle_exit(args):
    response = requests.get(get_logout_url())
    if response.status_code == 200 and b'"result":1' in response.content:
        notify("登出成功。")
    else:
        notify(f"登出失败。\n错误码: {response.status_code}\n错误信息: {response.content}")


if __name__ == "__main__":
    if not check_cnu():
        notify("请先连接校园网。")
        exit()

    parser = argparse.ArgumentParser(description="CNU Network Tool.")
    subparsers = parser.add_subparsers(help="commands")
    login_parser = subparsers.add_parser("login", help="登录校园网")
    login_parser.set_defaults(handle=handle_login)
    login_parser.add_argument(
        "-a", "--account", help="登录的校园网账号", required=True, metavar="account", type=str
    )
    login_parser.add_argument(
        "-p", "--password", help="登录的校园网密码", required=True, metavar="password", type=str
    )
    exit_parser = subparsers.add_parser("exit", help="登出校园网")
    exit_parser.set_defaults(handle=handle_exit)
    status_parser = subparsers.add_parser("status", help="查询校园网")
    status_parser.set_defaults(handle=handle_status)
    status_parser.add_argument(
        "-a", "--account", help="登录的校园网账号", required=True, metavar="account", type=str
    )

    args = parser.parse_args()
    if hasattr(args, "handle"):
        args.handle(args)
