#ifndef AXIS_COMMON_H_
#define AXIS_COMMON_H_

#include <algorithm>
#include <bits/stdc++.h>
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <exception>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <memory>
#include <numeric>
#include <ostream>
#include <streambuf>
#include <string>
#include <sys/stat.h>
#include <thread>
#include <unistd.h>
#include <vector>

#if __cplusplus <= 201703L
#include <experimental/filesystem>
#include <experimental/optional>
#else
#include <filesystem>
#include <optional>
#endif

#include <amqp.h>
#include <hiredis/hiredis.h>
#include <libpq-fe.h>

#include "env.h"

#endif
