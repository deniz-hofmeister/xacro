pub const TEST_INCLUDE_BASIC: &[u8] = br##"
<?xml version="1.0"?>
<robot name="test_robot" xmlns:xacro="http://www.ros.org/wiki/xacro">
    <!-- Include a basic component -->
    <xacro:include filename="base_component.xacro"/>
    
    <!-- Use the included content -->
    <link name="additional_link"/>
</robot>
"##;

pub const TEST_INCLUDE_BASE_COMPONENT: &[u8] = br##"
<?xml version="1.0"?>
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
    <link name="base_link">
        <visual>
            <geometry>
                <cylinder length="0.6" radius="0.2"/>
            </geometry>
        </visual>
    </link>
</robot>
"##;

pub const EXPECT_INCLUDE_BASIC: &[u8] = br##"
<?xml version="1.0"?>
<robot name="test_robot">
    <link name="base_link">
        <visual>
            <geometry>
                <cylinder length="0.6" radius="0.2"/>
            </geometry>
        </visual>
    </link>
    
    <link name="additional_link"/>
</robot>
"##;

// Nested includes test
pub const TEST_INCLUDE_NESTED: &[u8] = br##"
<?xml version="1.0"?>
<robot name="test_robot" xmlns:xacro="http://www.ros.org/wiki/xacro">
    <!-- Include arm which itself includes a gripper -->
    <xacro:include filename="robot_arm.xacro"/>
</robot>
"##;

pub const TEST_INCLUDE_ROBOT_ARM: &[u8] = br##"
<?xml version="1.0"?>
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
    <link name="arm_base"/>
    <link name="arm_link_1"/>
    
    <!-- Include the gripper component -->
    <xacro:include filename="gripper.xacro"/>
</robot>
"##;

pub const TEST_INCLUDE_GRIPPER: &[u8] = br##"
<?xml version="1.0"?>
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
    <link name="gripper_base"/>
    <link name="gripper_finger_1"/>
    <link name="gripper_finger_2"/>
</robot>
"##;

pub const EXPECT_INCLUDE_NESTED: &[u8] = br##"
<?xml version="1.0"?>
<robot name="test_robot">
    <link name="arm_base"/>
    <link name="arm_link_1"/>
    <link name="gripper_base"/>
    <link name="gripper_finger_1"/>
    <link name="gripper_finger_2"/>
</robot>
"##;

// Multiple includes test
pub const TEST_INCLUDE_MULTIPLE: &[u8] = br##"
<?xml version="1.0"?>
<robot name="test_robot" xmlns:xacro="http://www.ros.org/wiki/xacro">
    <!-- Include multiple components -->
    <xacro:include filename="wheels.xacro"/>
    <xacro:include filename="sensors.xacro"/>
</robot>
"##;

pub const TEST_INCLUDE_WHEELS: &[u8] = br##"
<?xml version="1.0"?>
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
    <link name="wheel_front_right"/>
    <link name="wheel_front_left"/>
</robot>
"##;

pub const TEST_INCLUDE_SENSORS: &[u8] = br##"
<?xml version="1.0"?>
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
    <link name="camera_link"/>
    <link name="lidar_link"/>
</robot>
"##;

pub const EXPECT_INCLUDE_MULTIPLE: &[u8] = br##"
<?xml version="1.0"?>
<robot name="test_robot">
    <link name="wheel_front_right"/>
    <link name="wheel_front_left"/>
    <link name="camera_link"/>
    <link name="lidar_link"/>
</robot>
"##;
