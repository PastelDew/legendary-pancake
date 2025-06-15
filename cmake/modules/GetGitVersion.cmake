function(GetGitVersion)
    find_package(Git QUIET)
    
    if(NOT GIT_FOUND)
        message(WARNING "Git not found, using default version")
        set(VERSION_MAJOR 0 PARENT_SCOPE)
        set(VERSION_MINOR 1 PARENT_SCOPE)
        set(VERSION_PATCH 0 PARENT_SCOPE)
        return()
    endif()
    
    # Git 정보 수집
    execute_process(
        COMMAND ${GIT_EXECUTABLE} describe --tags --abbrev=0
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE GIT_TAG
        OUTPUT_STRIP_TRAILING_WHITESPACE
        ERROR_QUIET
    )

    # 커밋 해시 가져오기
    execute_process(
        COMMAND ${GIT_EXECUTABLE} rev-parse --short HEAD
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE GIT_COMMIT_HASH
        OUTPUT_STRIP_TRAILING_WHITESPACE
        ERROR_QUIET
    )
    
    if(NOT GIT_TAG)
        set(GIT_TAG "v0.1.0")
    endif()
    
    # SemVer 파싱 (더 엄격한 정규식)
    string(REGEX MATCH 
        "^v?([0-9]+)\\.([0-9]+)\\.([0-9]+)(-((0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*)(\\.(0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*))*))?((\\+([0-9a-zA-Z-]+(\\.[0-9a-zA-Z-]+)*)))?$"
        SEMVER_MATCH "${GIT_TAG}")
    
    if(SEMVER_MATCH)
        set(VERSION_MAJOR ${CMAKE_MATCH_1} PARENT_SCOPE)
        set(VERSION_MINOR ${CMAKE_MATCH_2} PARENT_SCOPE)
        set(VERSION_PATCH ${CMAKE_MATCH_3} PARENT_SCOPE)
        set(VERSION_PRERELEASE ${CMAKE_MATCH_5} PARENT_SCOPE)
        set(GIT_COMMIT_HASH ${GIT_COMMIT_HASH} PARENT_SCOPE)
    else()
        message(WARNING "Invalid semantic version format: ${GIT_TAG}")
        set(VERSION_MAJOR 0 PARENT_SCOPE)
        set(VERSION_MINOR 1 PARENT_SCOPE)
        set(VERSION_PATCH 0 PARENT_SCOPE)
    endif()
endfunction()