#pragma once

namespace lpe
{
    namespace window
    {
        // Forward declaration of the WindowService class
        class WindowFoam
        {
        public:
            virtual ~IWindowService() = default;

            // Method to create a window with specified parameters
            virtual void createWindow(const char* title, int width, int height) = 0;

            // Method to destroy the current window
            virtual void destroyWindow() = 0;

            // Method to check if the window is open
            virtual bool isWindowOpen() const = 0;

            // Method to process window events
            virtual void processEvents() = 0;
        };
    } // namespace service
} // namespace lpe